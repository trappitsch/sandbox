import asyncio
import concurrent
import time

from async_serial_py.comm import Comm


async def send_cmd(inst, delay):
    tic = time.time()
    await inst.async_delay(delay)
    ack = f"ACK {delay}"
    toc = time.time()

    print(
        f"Time set: {delay} seconds\n"
        f"Time taken: {toc - tic:.2f} seconds\n"
        f"Acknowledgement: {ack}"
    )


async def run():
    inst0 = Comm("/dev/ttyACM0", 9600)
    inst1 = Comm("/dev/ttyACM1", 9600)

    tic = time.time()

    task1 = asyncio.create_task(send_cmd(inst0, 1))
    task2 = asyncio.create_task(send_cmd(inst1, 2))

    await task1
    await task2

    toc = time.time()

    print(f"Total time taken: {toc - tic:.2f} seconds")


def sync_send(inst, delay):
    tic = time.time()
    ack = inst.delay(delay)
    toc = time.time()

    print(
        f"Time set: {delay} seconds\n"
        f"Time taken: {toc - tic:.2f} seconds\n"
        f"Acknowledgement: {ack}"
    )


def sync_run():
    inst0 = Comm("/dev/ttyACM0", 9600)
    inst1 = Comm("/dev/ttyACM1", 9600)

    tic = time.time()

    sync_send(inst0, 1)
    sync_send(inst1, 2)

    toc = time.time()

    print(f"Total time taken: {toc - tic:.2f} seconds")


# sync_run()
asyncio.run(run())
