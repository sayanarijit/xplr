v="0.4.40"

curl -L https://github.com/rust-lang/mdBook/releases/download/v$v/mdbook-v$v-x86_64-unknown-linux-gnu.tar.gz -o mdbook.tgz \
  && tar xzvf mdbook.tgz \
  && ./mdbook build docs/en \
  && mkdir dist \
  && mv -v docs/en/book/html dist/en \
  && mv -v assets dist \
  && mv -v docs/landing/index.html docs/landing/css docs/landing/js dist \
  && rm -v mdbook \
  && rm -v mdbook.tgz
