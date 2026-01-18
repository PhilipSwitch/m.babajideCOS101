--
-- PostgreSQL database dump
--

\restrict RWBUaCaaiYHoFWqasNb1VbgL5ZKvCedXA1GvYVHADfFETb8WyVWTqm2oVwXXJCO

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
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    project_no integer NOT NULL,
    project_name text NOT NULL,
    project_duration text NOT NULL,
    project_manager_id integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_employee_id_not_null NOT NULL,
    staff_name text CONSTRAINT employees_employee_name_not_null NOT NULL,
    department_number integer CONSTRAINT employees_department_number_not_null NOT NULL,
    staff_salary numeric(10,2) CONSTRAINT employees_employee_salary_not_null NOT NULL,
    age integer,
    mobile character varying(15) NOT NULL,
    CONSTRAINT employees_age_check CHECK ((age >= 18))
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: customer_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer_table (c_id, c_name, c_age, c_email, c_mobile, e_id, data_id) FROM stdin;
110	Musta Karim		35	m_karim@gmail.com	 08055089112	102	5
111	Lilian Jaiya	43	I_jaiye@gmail.com	 08055185341	100	3
112	Arthur Musa		50	a_musa@gmail.com	 07055282813	10	10
113	Philip Akonjo	41	p_akonjo@gmail.com	 09052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	 08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	 07055566774	117	11
116	Adams Bree		33	a_bree@gmail.com	 08056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	 08056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	 07056774423	117	11
119	Lawal Tamire	35	I_tamire@gmail.com	 09052111101	107	5
120	James Job		44	j_job@gmail.com		 08059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	 07051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com 08054921923	107	5
\.


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
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (project_no, project_name, project_duration, project_manager_id) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, department_number, staff_salary, age, mobile) FROM stdin;
100	Mustapha Ali	3	175000.00	32	08063285831
107	Alokwe Martin	7	380000.00	48	07090082812
97	Dankade Aminat	5	550000.00	40	09023688832
108	Josiah Joshua	1	120000.00	30	08053189131
102	Makinde Mary	2	450000.00	55	09023487830
120	Adeleke Jane	4	200000.00	38	07061045682
122	Osahon Mark		6	320000.00	44	08022289842
104	Kuti Lawal		1	750000.00	35	09145689842
117	Suleman Ajayi	3	800000.00	50	7030089981
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
-- Name: dataplan_table dataplan_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan_table
    ADD CONSTRAINT dataplan_table_pkey PRIMARY KEY (data_id);


--
-- Name: staff employees_phone_number_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_phone_number_key UNIQUE (mobile);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (project_no);


--
-- PostgreSQL database dump complete
--

\unrestrict RWBUaCaaiYHoFWqasNb1VbgL5ZKvCedXA1GvYVHADfFETb8WyVWTqm2oVwXXJCO

