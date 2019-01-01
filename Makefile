all: build serve

serve:
	zola serve

build: build-zola fetch-src build-doc

build-zola:
	zola build

build-doc:
	./scripts/build_doc.sh

fetch-src:
	./scripts/fetch_doc_src.sh
