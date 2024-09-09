import asyncio
import time

import serial
import aioserial


class Comm:
    def __init__(self, port, baudrate):
        self.port, self.baudrate = port, baudrate
        self._conn = aioserial.AioSerial(port, baudrate)
        # self._conn = serial.Serial(port, baudrate)
        time.sleep(1)

    async def async_delay(self, dt: int) -> str:
        await self._conn.write_async(b"DELAY " + str(dt).encode() + b"\n")
        return await self._conn.readline_async()

    def delay(self, dt: int) -> str:
        """Get an acknowledgement back after a certain time.

        :param dt: Delay time in seconds.

        :return: Acknowledgement message as a string.
        """
        self._conn.write(b"DELAY " + str(dt).encode() + b"\n")
        return self._conn.readline().decode().strip()
