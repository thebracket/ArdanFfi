# If You Get Stuck

I've included a Dockerfile, `code/Dockerfile.ex01`.
so you can marvel at the splendour of Hello World in C!

You can run the Dockerfile with:

```bash
cd code/
docker build -t ffi . # You can also type "make"
docker run -it ffi

# You're now in a bash prompt. vim is available. You can run the examples in there.
```