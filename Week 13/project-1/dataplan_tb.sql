--
-- PostgreSQL database dump
--

\restrict x9BODaHRWcAd7bSIAERB1vBXEGaR6htqDlqPIjR8ha0S6sFjQEGAxVOAc7xc8DW

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
-- Name: dataplan_table; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan_table (
    data_id integer NOT NULL,
    data_size text NOT NULL,
    data_duration integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan_table OWNER TO postgres;

--
-- Data for Name: dataplan_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan_table (data_id, data_size, data_duration, data_price) FROM stdin;
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Name: dataplan_table dataplan_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan_table
    ADD CONSTRAINT dataplan_table_pkey PRIMARY KEY (data_id);


--
-- PostgreSQL database dump complete
--

\unrestrict x9BODaHRWcAd7bSIAERB1vBXEGaR6htqDlqPIjR8ha0S6sFjQEGAxVOAc7xc8DW

