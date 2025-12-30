#!/usr/bin/env python3
import asyncio
import sys
import json

try:
    from nats.aio.client import Client as NATS
except Exception:
    print("nats-py not installed; install with: python -m pip install --user nats-py")
    sys.exit(2)

async def main(outpath):
    nc = NATS()
    for attempt in range(8):
        try:
            await nc.connect("127.0.0.1:4222")
            break
        except Exception as e:
            if attempt == 7:
                print('Cannot connect to NATS:', e)
                sys.exit(1)
            await asyncio.sleep(1)
    msgs = []
    received = asyncio.Event()

    async def cb(msg):
        msgs.append(msg.data)
        received.set()
        await nc.close()

    await nc.subscribe("domain.events", cb=cb)

    try:
        await asyncio.wait_for(received.wait(), timeout=30)
    except asyncio.TimeoutError:
        pass

    if not msgs:
        print("No domain.events messages received within 30s")
        sys.exit(1)
    s = msgs[0].decode()
    print(s)
    with open(outpath, 'w') as f:
        f.write(s)

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print('Usage: fetch_signed_msg.py <outpath>')
        sys.exit(2)
    asyncio.run(main(sys.argv[1]))
