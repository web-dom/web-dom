everything: setup generate_webidl lint build minify examples
build:
	./node_modules/.bin/rollup js/web-dom.js --file web-dom.js --format umd --name WebDOM
	cargo fmt
.PHONY: examples
examples:
	cd examples/helloworld && make
	cd examples/alert && make
	cd examples/canvas && make
	cd examples/events && make
setup:
	npm install
generate_webidl:
	node tools/generate_json.js Console.webidl Window.webidl Document.webidl HTMLCanvasElement.webidl CanvasRenderingContext2D.webidl EventTarget.webidl KeyboardEvent.webidl MouseEvent.webidl Element.webidl HTMLInputElement.webidl WindowOrWorkerGlobalScope.webidl Node.webidl
	node tools/generate_webidl.js
lint:
	./node_modules/.bin/prettier --write js/web-dom.js js/webidl.js tools/generate_webidl.js
minify:
	./node_modules/.bin/babel-minify web-dom.js -o web-dom.min.js
publish:
	npm publish
