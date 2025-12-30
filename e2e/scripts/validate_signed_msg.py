#!/usr/bin/env python3
import json
import sys

if len(sys.argv) < 2:
    print('Usage: validate_signed_msg.py <path>')
    sys.exit(2)

p = sys.argv[1]
try:
    j = json.load(open(p))
except Exception as e:
    print('Failed to load JSON:', e)
    sys.exit(3)

if 'event' not in j or 'signature' not in j:
    print('Not a SignedEvent')
    sys.exit(2)
kind = j['event'].get('kind','')
if not kind.startswith('adapters.'):
    print('Unexpected event.kind:', kind)
    sys.exit(4)
if 'inmemory-discord' not in kind and 'discord' not in kind:
    print('Event.kind does not include expected adapter identifier:', kind)
    sys.exit(5)
print('Payload checks passed, kind=', kind)
