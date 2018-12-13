# blue-pill-quickstart [![Build status](https://travis-ci.org/TeXitoi/blue-pill-quickstart.svg?branch=master)](https://travis-ci.org/TeXitoi/blue-pill-quickstart)

Quickstart a Rust project for the [blue pill board](https://wiki.stm32duino.com/index.php?title=Blue_Pill), or any STM32F103xx board.

## Quickstart a new project

This section suppose your computer is already ready to hack on a blue pill.

Get and cleanup:

```shell
git clone https://github.com/TeXitoi/blue-pill-quickstart.git my-new-project
cd my-new-project
rm -fr .git LICENSE README.md st-link-v2-blue-pill.jpg
git init
```

Edit `Cargo.toml` for author and project name, and you're ready to go.

## Setting up everything on your machine

First, you need hardware. Buy a [blue pill](https://www.aliexpress.com/w/wholesale-stm32f103c8t6.html?&SortType=total_tranpro_desc) and a [ST-LINK V2](https://www.aliexpress.com/w/wholesale-st-link-v2.html?SortType=total_tranpro_desc). You also need a computer, I will suppose you have a Debian based distribution. It should be easy to adapt the instructions to any supported computer (Linux, MacOSX, Windows).

Then, install and setup everything on your computer:

```shell
curl https://sh.rustup.rs -sSf | sh
rustup target add thumbv7m-none-eabi
sudo apt-get install gdb-arm-none-eabi openocd
```

If you don't have `gdb-arm-none-eabi`, you can try `gdb-multiarch` (on Ubuntu 18.04 for example) or `gdb`. In these cases, you'll have to update `.cargo/config` accordingly.

Clone the repository:

```shell
git clone https://github.com/TeXitoi/blue-pill-quickstart.git
cd blue-pill-quickstart
```

Now, connect your ST-LINK to your blue pill. Connect the ST-LINK to your computer.

![ST-LINK V2 to blue pill](st-link-v2-blue-pill.jpg)

Launch openocd (the command may fail, go to "Trouble Shooting" for the potential solution):

```shell
./openocd.sh
```
 
Open a new terminal, compile and flash

```shell
cd blue-pill-quickstart
cargo run
```

Now, the program is flashed, and you are on a gdb prompt. Type `c` (for continue) you can see the on board LED blinking.

## Trouble Shooting

The formerly mentionned st-link may not have the right pin mapping as showed on its shell. If `openocd` returns `unknown code 0x9`, please check the pin mapping by removing the shell and re-connect your st-link with the mapping shown on the PCB.

If you're unable to remove the shell, try this pin mapping:

|pin|      |pin|       | 
|---|------|---|-------|
| 1 | RST  | 2 | SWCLK |
| 3 | SWIM | 4 | SWDIO |
| 5 | GND  | 6 | GND   |
| 7 | 3.3V | 8 | 3.3V  |
| 9 | 5.0V |10 | 5.0V  |

When flashing your blue pill for the first time, flashing may fail with the following messages in the openocd console:

```
Error: stm32x device protected
Error: failed erasing sectors 0 to 23
Error: flash_erase returned -4
```

This means your blue pill's flash is protected. To unlock it, you can connect to your openocd session with:

```shell
telnet localhost 4444
```

and type the following commands:

```
reset halt
stm32f1x unlock 0
reset halt
```

## Sources

This quickstart is inspired by the [cortex-m-quickstart](https://github.com/japaric/cortex-m-quickstart) and [Discovery](https://rust-embedded.github.io/discovery/). I recommand reading them.
