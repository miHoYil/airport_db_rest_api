--
-- PostgreSQL database dump
--

-- Dumped from database version 16.2
-- Dumped by pg_dump version 16.2

-- Started on 2024-03-17 22:39:41

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 234 (class 1255 OID 17432)
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO postgres;

--
-- TOC entry 235 (class 1255 OID 17433)
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 218 (class 1259 OID 17312)
-- Name: AirPlaneSeatTypes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."AirPlaneSeatTypes" (
    id_placa_type integer NOT NULL,
    type_name text NOT NULL
);


ALTER TABLE public."AirPlaneSeatTypes" OWNER TO postgres;

--
-- TOC entry 217 (class 1259 OID 17311)
-- Name: AirPlaneSeatType_id_placa_type_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."AirPlaneSeatTypes" ALTER COLUMN id_placa_type ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."AirPlaneSeatType_id_placa_type_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 215 (class 1259 OID 17303)
-- Name: AirPlanes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."AirPlanes" (
    id_plane integer NOT NULL,
    plane_name text NOT NULL
);


ALTER TABLE public."AirPlanes" OWNER TO postgres;

--
-- TOC entry 216 (class 1259 OID 17310)
-- Name: AirPlanes_id_plane_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."AirPlanes" ALTER COLUMN id_plane ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."AirPlanes_id_plane_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 224 (class 1259 OID 17349)
-- Name: AirPorts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."AirPorts" (
    id_airport integer NOT NULL,
    airport_name text NOT NULL,
    airport_address text NOT NULL
);


ALTER TABLE public."AirPorts" OWNER TO postgres;

--
-- TOC entry 223 (class 1259 OID 17348)
-- Name: AirPorts_id_airport_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."AirPorts" ALTER COLUMN id_airport ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."AirPorts_id_airport_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 221 (class 1259 OID 17340)
-- Name: Clients; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Clients" (
    id_client integer NOT NULL,
    name text NOT NULL,
    day_of_birth date NOT NULL,
    mail text NOT NULL,
    passport text NOT NULL
);


ALTER TABLE public."Clients" OWNER TO postgres;

--
-- TOC entry 222 (class 1259 OID 17347)
-- Name: Clients_id_client_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."Clients" ALTER COLUMN id_client ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Clients_id_client_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 226 (class 1259 OID 17357)
-- Name: FlightsSchedule; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."FlightsSchedule" (
    id_flight integer NOT NULL,
    id_plane integer NOT NULL,
    from_id_airport integer NOT NULL,
    to_id_airport integer NOT NULL,
    depature_date timestamp without time zone NOT NULL,
    arrive_date timestamp without time zone NOT NULL
);


ALTER TABLE public."FlightsSchedule" OWNER TO postgres;

--
-- TOC entry 225 (class 1259 OID 17356)
-- Name: FlightsSchedule_id_flight_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."FlightsSchedule" ALTER COLUMN id_flight ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."FlightsSchedule_id_flight_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 220 (class 1259 OID 17320)
-- Name: PlaneSeatConnections; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."PlaneSeatConnections" (
    id_plane_seat integer NOT NULL,
    id_place_type integer NOT NULL,
    id_plane integer NOT NULL,
    seat_number integer NOT NULL
);


ALTER TABLE public."PlaneSeatConnections" OWNER TO postgres;

--
-- TOC entry 219 (class 1259 OID 17319)
-- Name: PlaneSeatConnection_id_plane_seat_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."PlaneSeatConnections" ALTER COLUMN id_plane_seat ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."PlaneSeatConnection_id_plane_seat_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 232 (class 1259 OID 17419)
-- Name: ReceiptType; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."ReceiptType" (
    id_receipt_type integer NOT NULL,
    type_name text NOT NULL
);


ALTER TABLE public."ReceiptType" OWNER TO postgres;

--
-- TOC entry 231 (class 1259 OID 17418)
-- Name: ReceiptType_id_receipt_type_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."ReceiptType" ALTER COLUMN id_receipt_type ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."ReceiptType_id_receipt_type_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 230 (class 1259 OID 17413)
-- Name: Receipts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Receipts" (
    id_receipt integer NOT NULL,
    id_ticket integer NOT NULL,
    date_order timestamp without time zone NOT NULL,
    id_receipt_type integer NOT NULL
);


ALTER TABLE public."Receipts" OWNER TO postgres;

--
-- TOC entry 229 (class 1259 OID 17412)
-- Name: Receipts_id_receipt_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."Receipts" ALTER COLUMN id_receipt ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Receipts_id_receipt_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 227 (class 1259 OID 17392)
-- Name: Tickets; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Tickets" (
    id_ticket integer NOT NULL,
    id_flight integer NOT NULL,
    id_client integer NOT NULL,
    seat_number integer NOT NULL,
    baggage boolean NOT NULL,
    value_wo_b double precision NOT NULL,
    value_w_b double precision NOT NULL
);


ALTER TABLE public."Tickets" OWNER TO postgres;

--
-- TOC entry 228 (class 1259 OID 17411)
-- Name: Tickets_id_ticket_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

ALTER TABLE public."Tickets" ALTER COLUMN id_ticket ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Tickets_id_ticket_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 233 (class 1259 OID 17426)
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO postgres;

--
-- TOC entry 4854 (class 0 OID 17312)
-- Dependencies: 218
-- Data for Name: AirPlaneSeatTypes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."AirPlaneSeatTypes" (id_placa_type, type_name) FROM stdin;
1	Base
2	Comfort
3	Business
\.


--
-- TOC entry 4851 (class 0 OID 17303)
-- Dependencies: 215
-- Data for Name: AirPlanes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."AirPlanes" (id_plane, plane_name) FROM stdin;
1	Eagle
2	Storm
\.


--
-- TOC entry 4860 (class 0 OID 17349)
-- Dependencies: 224
-- Data for Name: AirPorts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."AirPorts" (id_airport, airport_name, airport_address) FROM stdin;
1	FUK	FUKUOKA, JP
2	SFO	SAN FRANCISCO CA, US
3	BWI	BALTIMORE MD
4	YYZ	TORONTO ON, CA
\.


--
-- TOC entry 4857 (class 0 OID 17340)
-- Dependencies: 221
-- Data for Name: Clients; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Clients" (id_client, name, day_of_birth, mail, passport) FROM stdin;
2	John	1980-07-31	2@mail.com	2000234567
3	Mark	2002-12-12	3@mail.com	3000345678
4	Alice	2001-04-24	4@mail.com	4000456789
1	-	1980-01-01	-	-
5	Mike	2000-01-12	1@mail.com	1000123456
\.


--
-- TOC entry 4862 (class 0 OID 17357)
-- Dependencies: 226
-- Data for Name: FlightsSchedule; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."FlightsSchedule" (id_flight, id_plane, from_id_airport, to_id_airport, depature_date, arrive_date) FROM stdin;
1	1	1	2	1999-01-08 04:05:06	1999-01-08 07:35:01
2	2	2	4	2024-03-17 07:03:06	2024-03-17 19:03:06
3	1	3	1	2022-06-12 06:06:06	2023-06-12 06:06:06
4	1	3	4	2020-06-12 05:31:23	2020-06-12 05:32:23
\.


--
-- TOC entry 4856 (class 0 OID 17320)
-- Dependencies: 220
-- Data for Name: PlaneSeatConnections; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."PlaneSeatConnections" (id_plane_seat, id_place_type, id_plane, seat_number) FROM stdin;
1	1	1	1
2	1	1	2
3	2	1	3
4	3	1	4
5	1	2	1
6	2	2	2
7	3	2	3
\.


--
-- TOC entry 4868 (class 0 OID 17419)
-- Dependencies: 232
-- Data for Name: ReceiptType; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."ReceiptType" (id_receipt_type, type_name) FROM stdin;
1	Cash
2	Credit Card
\.


--
-- TOC entry 4866 (class 0 OID 17413)
-- Dependencies: 230
-- Data for Name: Receipts; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Receipts" (id_receipt, id_ticket, date_order, id_receipt_type) FROM stdin;
1	5	2020-12-31 19:12:12	1
2	6	2019-11-12 21:23:12	2
3	7	2018-04-17 10:04:01	1
4	8	2012-05-23 05:31:23	2
5	9	2004-07-04 06:45:12	2
\.


--
-- TOC entry 4863 (class 0 OID 17392)
-- Dependencies: 227
-- Data for Name: Tickets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Tickets" (id_ticket, id_flight, id_client, seat_number, baggage, value_wo_b, value_w_b) FROM stdin;
1	2	1	1	f	12000.5	14000.5
2	2	1	2	f	12000.5	14000.5
3	2	1	3	f	15000	18000
4	2	1	4	f	25000	35000
5	1	2	1	f	1200	1400
6	1	2	2	f	4500	4900
7	3	3	1	t	150	180
8	4	4	1	f	14551	18231
9	4	5	2	t	23591	69420
\.


--
-- TOC entry 4869 (class 0 OID 17426)
-- Dependencies: 233
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2024-03-17 13:02:59.648714
\.


--
-- TOC entry 4875 (class 0 OID 0)
-- Dependencies: 217
-- Name: AirPlaneSeatType_id_placa_type_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."AirPlaneSeatType_id_placa_type_seq"', 3, true);


--
-- TOC entry 4876 (class 0 OID 0)
-- Dependencies: 216
-- Name: AirPlanes_id_plane_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."AirPlanes_id_plane_seq"', 2, true);


--
-- TOC entry 4877 (class 0 OID 0)
-- Dependencies: 223
-- Name: AirPorts_id_airport_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."AirPorts_id_airport_seq"', 4, true);


--
-- TOC entry 4878 (class 0 OID 0)
-- Dependencies: 222
-- Name: Clients_id_client_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Clients_id_client_seq"', 5, true);


--
-- TOC entry 4879 (class 0 OID 0)
-- Dependencies: 225
-- Name: FlightsSchedule_id_flight_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."FlightsSchedule_id_flight_seq"', 4, true);


--
-- TOC entry 4880 (class 0 OID 0)
-- Dependencies: 219
-- Name: PlaneSeatConnection_id_plane_seat_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."PlaneSeatConnection_id_plane_seat_seq"', 7, true);


--
-- TOC entry 4881 (class 0 OID 0)
-- Dependencies: 231
-- Name: ReceiptType_id_receipt_type_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."ReceiptType_id_receipt_type_seq"', 2, true);


--
-- TOC entry 4882 (class 0 OID 0)
-- Dependencies: 229
-- Name: Receipts_id_receipt_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Receipts_id_receipt_seq"', 5, true);


--
-- TOC entry 4883 (class 0 OID 0)
-- Dependencies: 228
-- Name: Tickets_id_ticket_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Tickets_id_ticket_seq"', 9, true);


--
-- TOC entry 4684 (class 2606 OID 17316)
-- Name: AirPlaneSeatTypes AirPlaneSeatType_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."AirPlaneSeatTypes"
    ADD CONSTRAINT "AirPlaneSeatType_pkey" PRIMARY KEY (id_placa_type);


--
-- TOC entry 4682 (class 2606 OID 17309)
-- Name: AirPlanes AirPlanes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."AirPlanes"
    ADD CONSTRAINT "AirPlanes_pkey" PRIMARY KEY (id_plane);


--
-- TOC entry 4690 (class 2606 OID 17355)
-- Name: AirPorts AirPorts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."AirPorts"
    ADD CONSTRAINT "AirPorts_pkey" PRIMARY KEY (id_airport);


--
-- TOC entry 4688 (class 2606 OID 17346)
-- Name: Clients Clients_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Clients"
    ADD CONSTRAINT "Clients_pkey" PRIMARY KEY (id_client);


--
-- TOC entry 4692 (class 2606 OID 17361)
-- Name: FlightsSchedule FlightsSchedule_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."FlightsSchedule"
    ADD CONSTRAINT "FlightsSchedule_pkey" PRIMARY KEY (id_flight);


--
-- TOC entry 4686 (class 2606 OID 17324)
-- Name: PlaneSeatConnections PlaneSeatConnection_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."PlaneSeatConnections"
    ADD CONSTRAINT "PlaneSeatConnection_pkey" PRIMARY KEY (id_plane_seat);


--
-- TOC entry 4698 (class 2606 OID 17425)
-- Name: ReceiptType ReceiptType_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."ReceiptType"
    ADD CONSTRAINT "ReceiptType_pkey" PRIMARY KEY (id_receipt_type);


--
-- TOC entry 4696 (class 2606 OID 17417)
-- Name: Receipts Receipts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Receipts"
    ADD CONSTRAINT "Receipts_pkey" PRIMARY KEY (id_receipt);


--
-- TOC entry 4694 (class 2606 OID 17396)
-- Name: Tickets Tickets_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Tickets"
    ADD CONSTRAINT "Tickets_pkey" PRIMARY KEY (id_ticket);


--
-- TOC entry 4700 (class 2606 OID 17431)
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- TOC entry 4703 (class 2606 OID 17372)
-- Name: FlightsSchedule FlightsSchedule_from_id_airport_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."FlightsSchedule"
    ADD CONSTRAINT "FlightsSchedule_from_id_airport_fkey" FOREIGN KEY (from_id_airport) REFERENCES public."AirPorts"(id_airport);


--
-- TOC entry 4704 (class 2606 OID 17362)
-- Name: FlightsSchedule FlightsSchedule_id_plane_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."FlightsSchedule"
    ADD CONSTRAINT "FlightsSchedule_id_plane_fkey" FOREIGN KEY (id_plane) REFERENCES public."AirPlanes"(id_plane);


--
-- TOC entry 4705 (class 2606 OID 17367)
-- Name: FlightsSchedule FlightsSchedule_to_id_airport_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."FlightsSchedule"
    ADD CONSTRAINT "FlightsSchedule_to_id_airport_fkey" FOREIGN KEY (to_id_airport) REFERENCES public."AirPorts"(id_airport);


--
-- TOC entry 4701 (class 2606 OID 17335)
-- Name: PlaneSeatConnections PlaneSeatConnection_id_place_type_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."PlaneSeatConnections"
    ADD CONSTRAINT "PlaneSeatConnection_id_place_type_fkey" FOREIGN KEY (id_place_type) REFERENCES public."AirPlaneSeatTypes"(id_placa_type) NOT VALID;


--
-- TOC entry 4702 (class 2606 OID 17330)
-- Name: PlaneSeatConnections PlaneSeatConnection_id_plane_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."PlaneSeatConnections"
    ADD CONSTRAINT "PlaneSeatConnection_id_plane_fkey" FOREIGN KEY (id_plane) REFERENCES public."AirPlanes"(id_plane) NOT VALID;


--
-- TOC entry 4706 (class 2606 OID 17402)
-- Name: Tickets Tickets_id_client_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Tickets"
    ADD CONSTRAINT "Tickets_id_client_fkey" FOREIGN KEY (id_client) REFERENCES public."Clients"(id_client);


--
-- TOC entry 4707 (class 2606 OID 17397)
-- Name: Tickets Tickets_id_flight_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Tickets"
    ADD CONSTRAINT "Tickets_id_flight_fkey" FOREIGN KEY (id_flight) REFERENCES public."FlightsSchedule"(id_flight);


-- Completed on 2024-03-17 22:39:41

--
-- PostgreSQL database dump complete
--

