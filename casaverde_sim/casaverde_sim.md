python3 -m venv venv
source venv/bin/activate.fish

pip install pyserial
  system wide: sudo pip install pyserial

Verify: Run python -c "import serial; print(serial.__version__)" (should output a version like 3.5).

Handle Permissions (Optional):
If pip install pyserial fails due to permissions in ~/.local, ensure your user has write access:
chmod -R u+rw ~/.local

pip install --upgrade pip

python --version  # Should show Python 3.x
pip --version     # Should show pip version for Python 3
python -c "import serial; print('pyserial installed:', sesial.__version__)"
