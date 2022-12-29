starerkit backend rust axum    

snippet list:  
- //get
- //routing_id
- //post
- //json_convert
- //get_header
- //sql_create
- //sql_update
- //sql_select_list
- //sql_count
- //web_socket

url list:  
http://localhost:3000/hello2
http://localhost:3000  

run code:  
cargo run  
cargo build  



<h1>script generate sample table</h1>


```
BEGIN;

CREATE TABLE IF NOT EXISTS public.albums_test (
	id bigint DEFAULT nextval('albums_id_seq'::regclass) NOT NULL,
	title character varying(255),
	subtitle character varying(255),
	privacy character varying(255),
	PRIMARY KEY(id)
);

COMMIT;
```
