--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name character varying(50) NOT NULL,
    c_age integer NOT NULL,
    c_email character varying(100) NOT NULL,
    c_mobile character varying(15) NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: customer_c_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.customer_c_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.customer_c_id_seq OWNER TO postgres;

--
-- Name: customer_c_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.customer_c_id_seq OWNED BY public.customer.c_id;


--
-- Name: customer c_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer ALTER COLUMN c_id SET DEFAULT nextval('public.customer_c_id_seq'::regclass);


--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055089112	102	5
111	Lilian Jaiya	43	l_jaiya@gmail.com	08055185341	100	3
112	Arthur Musa	50	a_musa@gmail.com	07055282813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	09052336772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	08053335551	100	2
115	Ogbonnaya Agor	30	o_agor@gmail.com	07055567744	117	11
116	Adams Bree	39	a_bree@gmail.com	08056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	08056733637	120	2
118	Samson Adeleke	46	s_adeleke@gmail.com	07056774423	117	11
119	Lawal Tamire	35	l_tamire@gmail.com	09052111101	107	5
120	James Job	44	j_job@gmail.com	09056939119	102	9
121	Matthew Jakande	20	m_jakande@gmail.com	07051232144	120	2
122	Jimila Adeboye	20	j_adeboye@gmail.com	08054921923	107	5
\.


--
-- Name: customer_c_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.customer_c_id_seq', 1, false);


--
-- Name: customer customer_c_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_c_email_key UNIQUE (c_email);


--
-- Name: customer customer_c_mobile_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_c_mobile_key UNIQUE (c_mobile);


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

