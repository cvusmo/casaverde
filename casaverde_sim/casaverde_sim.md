# create venv
python3 -m venv venv

# activate venv
source venv/bin/activate.fish

# install dependencies
pip install pyserial
  system wide: sudo pip install pyserial

# Verify: 
Run python -c "import serial; print(serial.__version__)" (should output a version like 3.5).

# Handle Permissions (Optional):
If pip install pyserial fails due to permissions in ~/.local, ensure your user has write access:
chmod -R u+rw ~/.local

# Upgrade
pip install --upgrade pip
python --version  # Should show Python 3.x
pip --version     # Should show pip version for Python 3
python -c "import serial; print('pyserial installed:', sesial.__version__)"

# check if casaverde is running:
ps aux | grep casaverde
kill -9 <PID>

# Clean
python /home/echo/projects/remote/casaverde/casaverde_sim/casaverde_automate.py --testing-root ~/casaverde_test --clean

# for testing:
python /home/echo/projects/remote/casaverde/casaverde_sim/casaverde_automate.py --testing-root ~/casaverde_test

# socat:
socat -d -d pty,raw,echo=0,link=/tmp/virtualcom0 pty,raw,echo=0,link=/tmp/virtualcom1 &

