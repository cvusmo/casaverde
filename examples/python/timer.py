# examples/python/timer.py
import RPi.GPIO as GPIO
import time

# Setup
RELAY_PIN = 17
GPIO.setmode(GPIO.BCM)
GPIO.setup(RELAY_PIN, GPIO.OUT)
GPIO.setwarnings(False)


# Active LOW (on when LOW, off when HIGH)
def light_on():
    GPIO.output(RELAY_PIN, GPIO.LOW)
    print("Light ON")


def light_off():
    GPIO.output(RELAY_PIN, GPIO.HIGH)
    print("Light OFF")


# Timer loop
try:
    while True:
        light_on()
        time.sleep(16 * 3600)  # 16 hours in seconds
        light_off()
        time.sleep(8 * 3600)  # 8 hours
except KeyboardInterrupt:
    GPIO.cleanup()
