# python casaverde_automate.py --testing-root ~/casaverde_test
# python /home/echo/projects/remote/casaverde/casaverde_sim/casaverde_automate.py --testing-root ~/casaverde_test

import argparse
import logging
import os
import shutil
import signal
import socket
import subprocess
import sys
import time


# Set up logging to file
def setup_logging(testing_root, log_level=logging.INFO):
    log_file = os.path.join(testing_root, "casaverde_automate.log")
    logging.basicConfig(
        level=log_level,
        format="%(asctime)s - %(levelname)s - %(message)s",
        handlers=[logging.FileHandler(log_file)],
    )
    logger = logging.getLogger(__name__)
    return logger


def check_path_exists(path, is_file=False):
    """Check if a path exists and matches the expected type."""
    if (is_file and not os.path.isfile(path)) or (
        not is_file and not os.path.isdir(path)
    ):
        raise FileNotFoundError(f"Path not found: {path}")


def is_port_open(host, port):
    """Check if a port is open on the given host."""
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(1)
    result = sock.connect_ex((host, port))
    sock.close()
    return result == 0


def main(args):
    """Main function to automate Casaverde testing setup and execution."""
    # Define paths with overrides from args/env
    home = os.path.expanduser("~")
    project_root = args.project_root or os.getenv(
        "PROJECT_ROOT", os.path.join(home, "projects", "remote", "casaverde")
    )
    testing_root = args.testing_root or os.getenv(
        "TESTING_ROOT", os.path.join(home, "casaverde_test")
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

    # Setup logging
    logger = setup_logging(testing_root, getattr(logging, args.log_level.upper()))

    # Validate key paths
    try:
        check_path_exists(project_root)
        check_path_exists(venv_python, is_file=True)
        check_path_exists(sim_script, is_file=True)
    except FileNotFoundError as e:
        logger.error(e)
        print(f"Error: {e}")
        sys.exit(1)

    # Clean testing root if requested
    if args.clean:
        if os.path.exists(testing_root):
            shutil.rmtree(testing_root)
        logger.info("Cleaned testing root.")
        print("Cleaned testing root.")

    # Create testing directories
    os.makedirs(testing_root, exist_ok=True)
    app_dir = os.path.join(testing_root, "casaverde_app")
    controller_dir = os.path.join(testing_root, "casaverde_controller")
    server_dir = os.path.join(testing_root, "casaverde_server")
    for dir_path in [app_dir, controller_dir, server_dir]:
        os.makedirs(dir_path, exist_ok=True)

    # Copy binaries from workspace root
    try:
        print("Copying binaries...", end=" ")
        for bin_name, dest in [
            ("casaverde_app", app_dir),
            ("casaverde_controller", controller_dir),
            ("casaverde_server", server_dir),
        ]:
            src = os.path.join(project_root, "target", "release", bin_name)
            shutil.copy(src, dest)
        logger.info("Binaries copied successfully.")
        print("Done.")
    except FileNotFoundError as e:
        logger.error(f"Binary copy failed: {e}")
        print("Failed.")
        sys.exit(1)

    # Copy and configure necessary files
    try:
        print("Copying config files and certificates...", end=" ")
        shutil.copy(os.path.join(project_root, "casaverde_app", "config.toml"), app_dir)
        shutil.copy(
            os.path.join(project_root, "casaverde_controller", "config.toml"),
            controller_dir,
        )
        for dir_path in [app_dir, controller_dir]:
            config_path = os.path.join(dir_path, "config.toml")
            with open(config_path, "r+") as f:
                text = f.read()
                text = text.replace("10.0.0.6:3003", "127.0.0.1:3003")
                f.seek(0)
                f.write(text)
                f.truncate()
        shutil.copy(os.path.join(config_dir, "server.crt"), app_dir)
        shutil.copy(os.path.join(config_dir, "server.crt"), controller_dir)
        logger.info("Config files and certificates copied successfully.")
        print("Done.")
    except FileNotFoundError as e:
        logger.error(f"File copy failed: {e}")
        print("Failed.")
        sys.exit(1)

    # Start processes
    processes = []

    # Check if port is free
    if is_port_open("127.0.0.1", 3003):
        logger.error(
            "Port 3003 is already in use. Stop the existing process and retry."
        )
        print("Error: Port 3003 is already in use.")
        sys.exit(1)

    # Start socat
    try:
        print("Starting socat...", end=" ")
        socat_cmd = [
            "socat",
            "-d",
            "-d",
            "pty,raw,echo=0,link=/tmp/virtualcom0",
            "pty,raw,echo=0,link=/tmp/virtualcom1",
        ]
        with open(os.path.join(testing_root, "socat.log"), "w") as log_file:
            socat_p = subprocess.Popen(
                socat_cmd, stdout=log_file, stderr=subprocess.STDOUT
            )
        processes.append(socat_p)
        time.sleep(2)
        logger.info("socat started.")
        print("Done.")
    except FileNotFoundError:
        logger.error("socat not found. Install it and retry.")
        print("Error: socat not found.")
        sys.exit(1)

    # Start simulator
    print("Starting simulator...", end=" ")
    with open(os.path.join(testing_root, "casaverde_sim.log"), "w") as sim_log:
        sim_p = subprocess.Popen(
            [venv_python, sim_script], stdout=sim_log, stderr=subprocess.STDOUT
        )
    processes.append(sim_p)
    time.sleep(1)
    logger.info("Simulator started.")
    print("Done.")

    # Start server
    print("Starting casaverde_server...", end=" ")
    server_bin = os.path.join(server_dir, "casaverde_server")
    env = os.environ.copy()
    env["SERVER_IP"] = "127.0.0.1:3003"
    with open(os.path.join(server_dir, "casaverde_server.log"), "w") as server_log:
        server_p = subprocess.Popen(
            [server_bin],
            cwd=server_dir,
            env=env,
            stdout=server_log,
            stderr=subprocess.STDOUT,
        )
    processes.append(server_p)
    start_time = time.time()
    while not is_port_open("127.0.0.1", 3003):
        if time.time() - start_time > 10:
            logger.error("Server failed to start on port 3003.")
            print("Failed.")
            sys.exit(1)
        time.sleep(1)
    logger.info("casaverde_server started.")
    print("Done.")

    # Start controller
    print("Starting casaverde_controller...", end=" ")
    controller_bin = os.path.join(controller_dir, "casaverde_controller")
    with open(
        os.path.join(controller_dir, "casaverde_controller.log"), "w"
    ) as controller_log:
        controller_p = subprocess.Popen(
            [controller_bin],
            cwd=controller_dir,
            stdout=controller_log,
            stderr=subprocess.STDOUT,
        )
    processes.append(controller_p)
    time.sleep(2)
    logger.info("casaverde_controller started.")
    print("Done.")

    # Start app
    print("Starting casaverde_app with --tui...", end=" ")
    app_bin = os.path.join(app_dir, "casaverde_app")
    with open(os.path.join(app_dir, "casaverde_app.log"), "w") as app_log:
        app_p = subprocess.Popen(
            [app_bin, "--tui"], cwd=app_dir, stdout=app_log, stderr=subprocess.STDOUT
        )
    processes.append(app_p)
    logger.info("casaverde_app started.")
    print("Done.")

    print(
        "\nAll components started successfully. Press Ctrl+C to stop.\nLogs are in the testing root directories."
    )

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("Stopping all processes...")
        for p in processes:
            p.send_signal(signal.SIGINT)
            p.wait(timeout=5)
        logger.info("Cleanup complete.")
        print("Cleanup complete. Check logs for details.")


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
    parser.add_argument(
        "--log-level", default="INFO", help="Set log level (DEBUG, INFO, etc.)"
    )
    args = parser.parse_args()
    main(args)
