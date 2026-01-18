--
-- PostgreSQL database dump
--

\restrict ckhVuXSIaKkBZK4QTF9GKlhQjLe58SPSy5SI2ee4tfGFLzxf58OZLuwkKY3CN5d

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
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, department_number, staff_salary, age, mobile) FROM stdin;
100	Mustapha Ali	3	175000.00	32	08063285831
107	Alokwe Martin	7	380000.00	48	07090082812
97	Dankade Aminat	5	550000.00	40	09023688832
108	Josiah Joshua	1	120000.00	30	08053189131
102	Makinde Mary	2	450000.00	55	09023487830
120	Adeleke Jane	4	200000.00	38	07061045682
122	Osahon Mark	    6	320000.00	44	08022289842
104	Kuti Lawal	    1	750000.00	35	09145689842
117	Suleman Ajayi	3	800000.00	50	7030089981
\.


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
-- PostgreSQL database dump complete
--

\unrestrict ckhVuXSIaKkBZK4QTF9GKlhQjLe58SPSy5SI2ee4tfGFLzxf58OZLuwkKY3CN5d

