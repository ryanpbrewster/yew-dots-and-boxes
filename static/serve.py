import SimpleHTTPServer
import SocketServer

PORT = 8080

class Handler(SimpleHTTPServer.SimpleHTTPRequestHandler):
    pass

Handler.extensions_map['.shtml'] = 'text/html'
Handler.extensions_map['.wasm'] = 'application/wasm'

httpd = SocketServer.TCPServer(("localhost", PORT), Handler)

print("serving at port", PORT)
httpd.serve_forever()
