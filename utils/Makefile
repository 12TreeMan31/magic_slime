CFLAGS = -O3 -Wall

client: client.c
	gcc client.c -o client.out $(CFLAGS) -lncurses
relay: relay.c
	gcc relay.c -o relay.out $(CFLAGS)

clean: 
	rm *.out