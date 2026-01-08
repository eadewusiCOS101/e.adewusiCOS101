--
-- PostgreSQL database dump
--

\restrict 4yhszq5ErVzC3diPzmnrxFveQn3ifem2uFB8A7lnIkbfuUp31vVl2lOEgeDTewc

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

-- Started on 2026-01-08 18:14:19

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
-- TOC entry 220 (class 1259 OID 16459)
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname character varying(255) NOT NULL,
    dlocation character varying(255),
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- TOC entry 219 (class 1259 OID 16451)
-- Name: employee; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.employee (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    dno integer NOT NULL,
    salary real,
    age integer,
    phone_number character(20)
);


ALTER TABLE public.employee OWNER TO postgres;

--
-- TOC entry 221 (class 1259 OID 16469)
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname character(2),
    pduration character varying(200),
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- TOC entry 4970 (class 0 OID 16459)
-- Dependencies: 220
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	Administration	Ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	V.I	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


--
-- TOC entry 4969 (class 0 OID 16451)
-- Dependencies: 219
-- Data for Name: employee; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.employee (id, name, dno, salary, age, phone_number) FROM stdin;
101	Alade Joy	2	250000	33	8023089832          
100	Mustapha Ali	3	175000	32	8063285831          
107	Alokwe Martin	7	380000	48	7090082812          
97	Dankade Aminat	5	55000	40	9023688832          
108	Josiah Joshua	1	120000	30	8053189131          
102	Makinde Mary	2	450000	55	9023487830          
120	Adekele Jane	4	200000	38	7061045862          
122	Osahon Mark	6	320000	44	8022289842          
117	Suleman Ajayi	3	800000	50	7030089811          
104	Kuti Lawal	1	750000	35	9145689842          
\.


--
-- TOC entry 4971 (class 0 OID 16469)
-- Dependencies: 221
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A 	9 months	102
22	B 	14 months	97
33	C 	16 months	120
44	D 	16 months	108
55	E 	9 months	107
\.


--
-- TOC entry 4819 (class 2606 OID 16468)
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dno);


--
-- TOC entry 4817 (class 2606 OID 16458)
-- Name: employee employee_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employee
    ADD CONSTRAINT employee_pkey PRIMARY KEY (id);


--
-- TOC entry 4821 (class 2606 OID 16475)
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pno);


-- Completed on 2026-01-08 18:14:19

--
-- PostgreSQL database dump complete
--

\unrestrict 4yhszq5ErVzC3diPzmnrxFveQn3ifem2uFB8A7lnIkbfuUp31vVl2lOEgeDTewc

