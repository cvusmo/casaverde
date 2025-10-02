# python casaverde_automate.py --testing-root ~/casaverde_test

import argparse
import logging
import os
import shutil
import signal
import socket
import subprocess
import sys
import time

# Set up logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


def check_path_exists(path, is_file=False):
    if (is_file and not os.path.isfile(path)) or (
        not is_file and not os.path.isdir(path)
    ):
        raise FileNotFoundError(f"Path not found: {path}")


def is_port_open(host, port):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(1)
    result = sock.connect_ex((host, port))
    sock.close()
    return result == 0


def main(args):
    # Define paths with overrides from args/env
    home = os.path.expanduser("~")
    project_root = args.project_root or os.getenv(
        "PROJECT_ROOT", os.path.join(home, "projects", "remote", "casaverde")
    )
    testing_root = args.testing_root or os.getenv(
        "TESTING_ROOT", os.path.join(home, "casaverde_testing")
    )
    venv_python = args.venv_python or os.getenv(
        "VENV_PYTHON",
        os.path.join(project_root, "casaverde_sim", "venv", "bin", "python"),
    )
    sim_script = args.sim_script or os.getenv(
        "SIM_SCRIPT", os.path.join(project_root, "casaverde_sim", "casaverde_sim_1.py")
    )
    config_dir = args.config_dir or os.getenv(
        "CONFIG_DIR", os.path.join(home, ".config", "casaverde_server")
    )

    # Validate key paths
    try:
        check_path_exists(project_root)
        check_path_exists(venv_python, is_file=True)
        check_path_exists(sim_script, is_file=True)
    except FileNotFoundError as e:
        logger.error(e)
        sys.exit(1)

    # Clean testing root if requested
    if args.clean:
        if os.path.exists(testing_root):
            shutil.rmtree(testing_root)
        logger.info("Cleaned testing root.")

    # Create testing directories
    os.makedirs(testing_root, exist_ok=True)
    app_dir = os.path.join(testing_root, "casaverde_app")
    controller_dir = os.path.join(testing_root, "casaverde_controller")
    server_dir = os.path.join(testing_root, "casaverde_server")
    os.makedirs(app_dir, exist_ok=True)
    os.makedirs(controller_dir, exist_ok=True)
    os.makedirs(server_dir, exist_ok=True)

    # Step 3: Build the projects with cargo
    for proj in ["casaverde_app", "casaverde_controller", "casaverde_server"]:
        try:
            logger.info(f"Building {proj}...")
            subprocess.check_call(
                ["cargo", "build", "--release"], cwd=os.path.join(project_root, proj)
            )
        except subprocess.CalledProcessError as e:
            logger.error(f"Build failed for {proj}: {e}")
            sys.exit(1)

    # Copy binaries to testing dirs
    try:
        shutil.copy(
            os.path.join(
                project_root, "casaverde_app", "target", "release", "casaverde_app"
            ),
            app_dir,
        )
        shutil.copy(
            os.path.join(
                project_root,
                "casaverde_controller",
                "target",
                "release",
                "casaverde_controller",
            ),
            controller_dir,
        )
        shutil.copy(
            os.path.join(
                project_root,
                "casaverde_server",
                "target",
                "release",
                "casaverde_server",
            ),
            server_dir,
        )
    except FileNotFoundError as e:
        logger.error(f"Binary copy failed: {e}")
        sys.exit(1)

    # Step 4: Run setup.sh for certificates
    setup_script = os.path.join(project_root, "casaverde_server", "setup.sh")
    try:
        check_path_exists(setup_script, is_file=True)
        logger.info("Running setup.sh...")
        subprocess.check_call(["bash", setup_script])
    except (FileNotFoundError, subprocess.CalledProcessError) as e:
        logger.error(f"setup.sh failed: {e}")
        sys.exit(1)

    # Step 6: Copy necessary files
    try:
        shutil.copy(os.path.join(project_root, "casaverde_app", "config.toml"), app_dir)
        shutil.copy(
            os.path.join(project_root, "casaverde_controller", "config.toml"),
            controller_dir,
        )
        # Update config.toml to use local server URL
        for dir_path in [app_dir, controller_dir]:
            config_path = os.path.join(dir_path, "config.toml")
            with open(config_path, "r+") as f:
                text = f.read()
                text = text.replace("10.0.0.6:3003", "127.0.0.1:3003")
                f.seek(0)
                f.write(text)
                f.truncate()
        # Copy server.crt
        shutil.copy(os.path.join(config_dir, "server.crt"), app_dir)
        shutil.copy(os.path.join(config_dir, "server.crt"), controller_dir)
    except FileNotFoundError as e:
        logger.error(f"File copy failed: {e}")
        sys.exit(1)

    # Step 7: Start processes
    processes = []

    # Check if port is free
    if is_port_open("127.0.0.1", 3003):
        logger.error(
            "Port 3003 is already in use. Stop the existing process and retry."
        )
        sys.exit(1)

    # Start socat (step 1)
    try:
        logger.info("Starting socat...")
        socat_cmd = [
            "socat",
            "-d",
            "-d",
            "pty,raw,echo=0,link=/tmp/virtualcom0",
            "pty,raw,echo=0,link=/tmp/virtualcom1",
        ]
        socat_p = subprocess.Popen(socat_cmd)
        processes.append(socat_p)
        time.sleep(2)  # Give time to initialize
    except FileNotFoundError:
        logger.error("socat not found. Install it and retry.")
        sys.exit(1)

    # Start simulator in venv (step 2)
    logger.info("Starting simulator...")
    sim_p = subprocess.Popen([venv_python, sim_script])
    processes.append(sim_p)
    time.sleep(1)

    # Start server with local IP
    logger.info("Starting casaverde_server...")
    server_bin = os.path.join(server_dir, "casaverde_server")
    env = os.environ.copy()
    env["SERVER_IP"] = "127.0.0.1:3003"
    server_p = subprocess.Popen([server_bin], cwd=server_dir, env=env)
    processes.append(server_p)
    # Poll for server readiness
    start_time = time.time()
    while not is_port_open("127.0.0.1", 3003):
        if time.time() - start_time > 10:
            logger.error("Server failed to start on port 3003.")
            sys.exit(1)
        time.sleep(1)

    # Start controller
    logger.info("Starting casaverde_controller...")
    controller_bin = os.path.join(controller_dir, "casaverde_controller")
    controller_p = subprocess.Popen([controller_bin], cwd=controller_dir)
    processes.append(controller_p)
    time.sleep(2)

    # Start app with --tui
    logger.info("Starting casaverde_app with --tui...")
    app_bin = os.path.join(app_dir, "casaverde_app")
    app_p = subprocess.Popen([app_bin, "--tui"], cwd=app_dir)
    processes.append(app_p)

    logger.info("All components started. Press Ctrl+C to stop.")

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        logger.info("Stopping all processes...")
        for p in processes:
            p.send_signal(signal.SIGINT)
            p.wait(timeout=5)  # Give time to shutdown
        logger.info("Cleanup complete.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Automate Casaverde testing setup and run."
    )
    parser.add_argument("--project-root", help="Override PROJECT_ROOT path")
    parser.add_argument("--testing-root", help="Override TESTING_ROOT path")
    parser.add_argument("--venv-python", help="Override VENV_PYTHON path")
    parser.add_argument("--sim-script", help="Override SIM_SCRIPT path")
    parser.add_argument("--config-dir", help="Override CONFIG_DIR path")
    parser.add_argument(
        "--clean", action="store_true", help="Clean testing root before starting"
    )
    args = parser.parse_args()
    main(args)
