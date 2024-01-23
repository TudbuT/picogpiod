# PicoGPIOd

*Part of [Pico_GPIO](https://github.com/tudbut/pico_gpio): A project to turn the cheap raspberry pi pico microcontroller into a 
decently powerful GPIO port.*

A PicoGPIO daemon for other programs (incl. CLI) to interact with the GPIO quickly and simultaneously.

## Basic commands

```sh
pgpio high 25 # enable LED
pgpio low 25 # disable LED
pgpio getpdn 0 # is pin 0 high or low?

# demo of possible use for getpdn:
if [ "$(pgpio getpdn 0)" = "HIGH" ] ; then
  echo "pin 0 is high"
fi
```

---

*This software was developed without the use of auto-plagiarizers like ChatGPT and GitHub Copilot.*
