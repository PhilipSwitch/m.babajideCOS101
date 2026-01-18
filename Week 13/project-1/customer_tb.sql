--
-- PostgreSQL database dump
--

\restrict 1C2nWRMwQyHedGyeQbT0lTRvXlZpyRKtLD3l7AC4gzRGz1lG43qAdsAD9AOqyEN

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

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
-- Name: customer_table; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer_table (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text NOT NULL,
    c_mobile character varying(25) NOT NULL,
    e_id integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer_table OWNER TO postgres;

--
-- Data for Name: customer_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer_table (c_id, c_name, c_age, c_email, c_mobile, e_id, data_id) FROM stdin;
110	Musta Karim	    35	m_karim@gmail.com	 08055089112	102	5
111	Lilian Jaiya	43	I_jaiye@gmail.com	 08055185341	100	3
112	Arthur Musa	    50	a_musa@gmail.com	 07055282813	10	10
113	Philip Akonjo	41	p_akonjo@gmail.com	 09052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	 08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	 07055566774	117	11
116	Adams Bree	    33	a_bree@gmail.com	 08056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	 08056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	 07056774423	117	11
119	Lawal Tamire	35	I_tamire@gmail.com	 09052111101	107	5
120	James Job	    44	j_job@gmail.com	     08059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	 07051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com 08054921923	107	5
\.


--
-- Name: customer_table customer_table_c_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer_table
    ADD CONSTRAINT customer_table_c_email_key UNIQUE (c_email);


--
-- Name: customer_table customer_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer_table
    ADD CONSTRAINT customer_table_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

\unrestrict 1C2nWRMwQyHedGyeQbT0lTRvXlZpyRKtLD3l7AC4gzRGz1lG43qAdsAD9AOqyEN

