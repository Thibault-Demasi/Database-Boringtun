------------------------------------------------------------
--        Script Postgre 
------------------------------------------------------------



------------------------------------------------------------
-- Table: vpnUser
------------------------------------------------------------
CREATE TABLE public.vpnUser(
	id_user             SERIAL NOT NULL ,
	email               VARCHAR (50) NOT NULL ,
	verified_email      BOOL  NOT NULL ,
	private_salt        VARCHAR (50) NOT NULL ,
	crypted_password    VARCHAR (150) NOT NULL ,
	admin               BOOL  NOT NULL ,
	public_key          VARCHAR (50)  ,
	private_key         VARCHAR (50)  ,
	interface_address   VARCHAR (50)  ,
	created_at	        TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP  NOT NULL  ,
	CONSTRAINT vpnUser_PK PRIMARY KEY (id_user)
)WITHOUT OIDS;


------------------------------------------------------------
-- Table: interface
------------------------------------------------------------
CREATE TABLE public.interface(
	id_interface     SERIAL NOT NULL ,
	DNS              VARCHAR (50) NOT NULL ,
	listen_port      INT  NOT NULL ,
	interface_name   VARCHAR (1000) NOT NULL ,
	id_user          INT  NOT NULL  ,
	CONSTRAINT interface_PK PRIMARY KEY (id_interface)

	,CONSTRAINT interface_vpnUser_FK FOREIGN KEY (id_user) REFERENCES public.vpnUser(id_user)
)WITHOUT OIDS;


------------------------------------------------------------
-- Table: peer
------------------------------------------------------------
CREATE TABLE public.peer(
	id_peer        SERIAL NOT NULL ,
	allowed_ips    VARCHAR (1000) NOT NULL ,
	endpoint       VARCHAR (50) NOT NULL ,
	public_key     VARCHAR (50) NOT NULL ,
	id_interface   INT  NOT NULL  ,
	CONSTRAINT peer_PK PRIMARY KEY (id_peer)

	,CONSTRAINT peer_interface_FK FOREIGN KEY (id_interface) REFERENCES public.interface(id_interface)
)WITHOUT OIDS;



