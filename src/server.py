import socket, threading, json, traceback, signal,sys

MAX_PLAYERS = 4


# request example:
# {
#     "op": "update"
#     "data": {
#         "vel": [0,0]
#         "pos": [0,0]
#         "world": 0
#      }
#  }
# world is either 0 or 1

# response example
# {
#   [
#     {
#       "pos": [0,0]
#       "vel": [0,0]
#       "id": 5
#       "world": 0
#   ]
# }
#


class EstablishedConnection:
    cid = 0
    instances = set()

    def __init__(self, conn: socket.socket, addr):
        self.conn = conn
        self.addr = addr
        self.id = self.cid
        self.vel = [0, 0]
        self.pos = [0, 0]
        self.world = 0
        self.conn.send(f'{self.id}\n'.encode('ascii'))
        print('connected', self.id)
        EstablishedConnection.instances.add(self)
        EstablishedConnection.cid += 1

    def __eq__(self, other):
        return self.id == other.id

    def __hash__(self):
        return self.id

    def loop(self):
        while True:
            try:
                data = ''
                while not data:
                    data += self.conn.recv(4096).decode('ascii')
                obj = json.loads(data.strip().split("\n")[-1])
                op = obj['op']
                if op == 'disconnect':
                    print('disconnected', self.id)
                    self.instances.remove(self)
                    break
                elif op == 'update':
                    print('update', obj['data'])
                    self.vel = obj['data']['vel']
                    self.pos = obj['data']['pos']
                    self.world = obj['data']['world']
                    resp = []
                    for o in filter(lambda i: i != self, self.instances):
                        resp.append({
                            'id'   : o.id,
                            'world': o.world,
                            'pos'  : o.pos,
                            'vel'  : o.vel
                        })
                    resp.sort(key=lambda o:o['id'])
                    self.conn.send((json.dumps(resp) + '\n').encode('ascii'))
            except (OSError, ConnectionResetError):
                self.instances.remove(self)
                self.stop()
                return
            except json.decoder.JSONDecodeError:
                print("Cannot decode")
                print(data.encode("ascii"))
            except:
                #raise
                print(traceback.format_exc(),file = sys.stderr)

    def stop(self):
        self.conn.shutdown(socket.SHUT_RDWR)
        self.conn.close()

    @classmethod
    def close_all(cls):
        for instance in cls.instances:
            instance.stop()


server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEPORT, 1)
server.bind(('0.0.0.0', 16512))
server.listen(4)


def handle_close(signum, frame):
    EstablishedConnection.close_all()
    server.close()


signal.signal(signal.SIGINT, handle_close)
signal.signal(signal.SIGTERM, handle_close)

while True:
    try:
        conn, addr = server.accept()
    except OSError:
        break
    est = EstablishedConnection(conn, addr)
    threading.Thread(target = est.loop, args = ()).start()

