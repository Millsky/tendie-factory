FROM jdrouet/rust-nightly:buster-slim as build-env
RUN apt-get update && apt-get install -y pkg-config libssl-dev g++
WORKDIR /app
ADD ./core /app
RUN cargo build --release
ENV LD_LIBRARY_PATH=/app/target/release/build/torch-sys-1371089814c1dca7/out/libtorch/libtorch/lib

RUN mkdir -p ~/.cache/.rustbert/bert-ner
RUN apt-get install -y curl
# RUN cd ~/.cache/.rustbert/bert-ner && mkdir config && cd config && curl -O https://huggingface.co/dbmdz/bert-large-cased-finetuned-conll03-english/resolve/main/config.json \
#    && cd ../ && mkdir vocab && cd vocab && curl -O https://huggingface.co/dbmdz/bert-large-cased-finetuned-conll03-english/resolve/main/vocab.txt \
#    && cd ../ && mkdir model && cd model && curl -O https://huggingface.co/dbmdz/bert-large-cased-finetuned-conll03-english/resolve/main/rust_model.ot
CMD ["/app/target/release/tendie-factory"]