PGDMP                         {         
   ProfolioDB    15.3    15.3 1    6           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                      false            7           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                      false            8           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                      false            9           1262    16398 
   ProfolioDB    DATABASE        CREATE DATABASE "ProfolioDB" WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'English_India.1252';
    DROP DATABASE "ProfolioDB";
                postgres    false            �            1259    16858 	   community    TABLE     �   CREATE TABLE public.community (
    id integer NOT NULL,
    uuid uuid NOT NULL,
    user_id integer NOT NULL,
    title character varying NOT NULL,
    description character varying
);
    DROP TABLE public.community;
       public         heap    postgres    false            �            1259    16857    community_id_seq    SEQUENCE     �   CREATE SEQUENCE public.community_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 '   DROP SEQUENCE public.community_id_seq;
       public          postgres    false    222            :           0    0    community_id_seq    SEQUENCE OWNED BY     E   ALTER SEQUENCE public.community_id_seq OWNED BY public.community.id;
          public          postgres    false    221            �            1259    16873    community_user    TABLE     �   CREATE TABLE public.community_user (
    id integer NOT NULL,
    user_id integer NOT NULL,
    community_id integer NOT NULL,
    type character varying NOT NULL,
    created_at timestamp without time zone NOT NULL
);
 "   DROP TABLE public.community_user;
       public         heap    postgres    false            �            1259    16872    community_user_id_seq    SEQUENCE     �   CREATE SEQUENCE public.community_user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 ,   DROP SEQUENCE public.community_user_id_seq;
       public          postgres    false    224            ;           0    0    community_user_id_seq    SEQUENCE OWNED BY     O   ALTER SEQUENCE public.community_user_id_seq OWNED BY public.community_user.id;
          public          postgres    false    223            �            1259    16804    seaql_migrations    TABLE     q   CREATE TABLE public.seaql_migrations (
    version character varying NOT NULL,
    applied_at bigint NOT NULL
);
 $   DROP TABLE public.seaql_migrations;
       public         heap    postgres    false            �            1259    16827 
   technology    TABLE     �   CREATE TABLE public.technology (
    id integer NOT NULL,
    uuid uuid NOT NULL,
    title character varying NOT NULL,
    normalized_title character varying NOT NULL
);
    DROP TABLE public.technology;
       public         heap    postgres    false            �            1259    16826    technology_id_seq    SEQUENCE     �   CREATE SEQUENCE public.technology_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 (   DROP SEQUENCE public.technology_id_seq;
       public          postgres    false    218            <           0    0    technology_id_seq    SEQUENCE OWNED BY     G   ALTER SEQUENCE public.technology_id_seq OWNED BY public.technology.id;
          public          postgres    false    217            �            1259    16812    user    TABLE     �  CREATE TABLE public."user" (
    id integer NOT NULL,
    uuid uuid NOT NULL,
    name character varying NOT NULL,
    email character varying NOT NULL,
    password character varying NOT NULL,
    phone character varying NOT NULL,
    phone_code integer NOT NULL,
    profession character varying,
    ctc integer DEFAULT 0 NOT NULL,
    experience integer DEFAULT 0 NOT NULL,
    company character varying,
    linkedin character varying,
    github character varying,
    others character varying
);
    DROP TABLE public."user";
       public         heap    postgres    false            �            1259    16811    user_id_seq    SEQUENCE     �   CREATE SEQUENCE public.user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 "   DROP SEQUENCE public.user_id_seq;
       public          postgres    false    216            =           0    0    user_id_seq    SEQUENCE OWNED BY     =   ALTER SEQUENCE public.user_id_seq OWNED BY public."user".id;
          public          postgres    false    215            �            1259    16840    user_technology    TABLE     �   CREATE TABLE public.user_technology (
    id integer NOT NULL,
    technology_id integer NOT NULL,
    user_id integer NOT NULL,
    score double precision NOT NULL
);
 #   DROP TABLE public.user_technology;
       public         heap    postgres    false            �            1259    16839    user_technology_id_seq    SEQUENCE     �   CREATE SEQUENCE public.user_technology_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 -   DROP SEQUENCE public.user_technology_id_seq;
       public          postgres    false    220            >           0    0    user_technology_id_seq    SEQUENCE OWNED BY     Q   ALTER SEQUENCE public.user_technology_id_seq OWNED BY public.user_technology.id;
          public          postgres    false    219            �           2604    16861    community id    DEFAULT     l   ALTER TABLE ONLY public.community ALTER COLUMN id SET DEFAULT nextval('public.community_id_seq'::regclass);
 ;   ALTER TABLE public.community ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    221    222    222            �           2604    16876    community_user id    DEFAULT     v   ALTER TABLE ONLY public.community_user ALTER COLUMN id SET DEFAULT nextval('public.community_user_id_seq'::regclass);
 @   ALTER TABLE public.community_user ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    224    223    224            �           2604    16830    technology id    DEFAULT     n   ALTER TABLE ONLY public.technology ALTER COLUMN id SET DEFAULT nextval('public.technology_id_seq'::regclass);
 <   ALTER TABLE public.technology ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    217    218    218            }           2604    16815    user id    DEFAULT     d   ALTER TABLE ONLY public."user" ALTER COLUMN id SET DEFAULT nextval('public.user_id_seq'::regclass);
 8   ALTER TABLE public."user" ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    216    215    216            �           2604    16843    user_technology id    DEFAULT     x   ALTER TABLE ONLY public.user_technology ALTER COLUMN id SET DEFAULT nextval('public.user_technology_id_seq'::regclass);
 A   ALTER TABLE public.user_technology ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    220    219    220            1          0    16858 	   community 
   TABLE DATA           J   COPY public.community (id, uuid, user_id, title, description) FROM stdin;
    public          postgres    false    222   �8       3          0    16873    community_user 
   TABLE DATA           U   COPY public.community_user (id, user_id, community_id, type, created_at) FROM stdin;
    public          postgres    false    224   �8       )          0    16804    seaql_migrations 
   TABLE DATA           ?   COPY public.seaql_migrations (version, applied_at) FROM stdin;
    public          postgres    false    214   9       -          0    16827 
   technology 
   TABLE DATA           G   COPY public.technology (id, uuid, title, normalized_title) FROM stdin;
    public          postgres    false    218   �9       +          0    16812    user 
   TABLE DATA           �   COPY public."user" (id, uuid, name, email, password, phone, phone_code, profession, ctc, experience, company, linkedin, github, others) FROM stdin;
    public          postgres    false    216   |:       /          0    16840    user_technology 
   TABLE DATA           L   COPY public.user_technology (id, technology_id, user_id, score) FROM stdin;
    public          postgres    false    220   ;       ?           0    0    community_id_seq    SEQUENCE SET     ?   SELECT pg_catalog.setval('public.community_id_seq', 1, false);
          public          postgres    false    221            @           0    0    community_user_id_seq    SEQUENCE SET     D   SELECT pg_catalog.setval('public.community_user_id_seq', 1, false);
          public          postgres    false    223            A           0    0    technology_id_seq    SEQUENCE SET     ?   SELECT pg_catalog.setval('public.technology_id_seq', 5, true);
          public          postgres    false    217            B           0    0    user_id_seq    SEQUENCE SET     9   SELECT pg_catalog.setval('public.user_id_seq', 1, true);
          public          postgres    false    215            C           0    0    user_technology_id_seq    SEQUENCE SET     D   SELECT pg_catalog.setval('public.user_technology_id_seq', 2, true);
          public          postgres    false    219            �           2606    16865    community community_pkey 
   CONSTRAINT     V   ALTER TABLE ONLY public.community
    ADD CONSTRAINT community_pkey PRIMARY KEY (id);
 B   ALTER TABLE ONLY public.community DROP CONSTRAINT community_pkey;
       public            postgres    false    222            �           2606    16880 "   community_user community_user_pkey 
   CONSTRAINT     `   ALTER TABLE ONLY public.community_user
    ADD CONSTRAINT community_user_pkey PRIMARY KEY (id);
 L   ALTER TABLE ONLY public.community_user DROP CONSTRAINT community_user_pkey;
       public            postgres    false    224            �           2606    16810 &   seaql_migrations seaql_migrations_pkey 
   CONSTRAINT     i   ALTER TABLE ONLY public.seaql_migrations
    ADD CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version);
 P   ALTER TABLE ONLY public.seaql_migrations DROP CONSTRAINT seaql_migrations_pkey;
       public            postgres    false    214            �           2606    16838 *   technology technology_normalized_title_key 
   CONSTRAINT     q   ALTER TABLE ONLY public.technology
    ADD CONSTRAINT technology_normalized_title_key UNIQUE (normalized_title);
 T   ALTER TABLE ONLY public.technology DROP CONSTRAINT technology_normalized_title_key;
       public            postgres    false    218            �           2606    16834    technology technology_pkey 
   CONSTRAINT     X   ALTER TABLE ONLY public.technology
    ADD CONSTRAINT technology_pkey PRIMARY KEY (id);
 D   ALTER TABLE ONLY public.technology DROP CONSTRAINT technology_pkey;
       public            postgres    false    218            �           2606    16836    technology technology_uuid_key 
   CONSTRAINT     Y   ALTER TABLE ONLY public.technology
    ADD CONSTRAINT technology_uuid_key UNIQUE (uuid);
 H   ALTER TABLE ONLY public.technology DROP CONSTRAINT technology_uuid_key;
       public            postgres    false    218            �           2606    16825    user user_email_key 
   CONSTRAINT     Q   ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_email_key UNIQUE (email);
 ?   ALTER TABLE ONLY public."user" DROP CONSTRAINT user_email_key;
       public            postgres    false    216            �           2606    16821    user user_pkey 
   CONSTRAINT     N   ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);
 :   ALTER TABLE ONLY public."user" DROP CONSTRAINT user_pkey;
       public            postgres    false    216            �           2606    16845 $   user_technology user_technology_pkey 
   CONSTRAINT     b   ALTER TABLE ONLY public.user_technology
    ADD CONSTRAINT user_technology_pkey PRIMARY KEY (id);
 N   ALTER TABLE ONLY public.user_technology DROP CONSTRAINT user_technology_pkey;
       public            postgres    false    220            �           2606    16823    user user_uuid_key 
   CONSTRAINT     O   ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_uuid_key UNIQUE (uuid);
 >   ALTER TABLE ONLY public."user" DROP CONSTRAINT user_uuid_key;
       public            postgres    false    216            �           2606    16866    community fk-community-user-id    FK CONSTRAINT     �   ALTER TABLE ONLY public.community
    ADD CONSTRAINT "fk-community-user-id" FOREIGN KEY (user_id) REFERENCES public."user"(id);
 J   ALTER TABLE ONLY public.community DROP CONSTRAINT "fk-community-user-id";
       public          postgres    false    3209    222    216            �           2606    16846 $   user_technology fk-user_tech-tech-id    FK CONSTRAINT     �   ALTER TABLE ONLY public.user_technology
    ADD CONSTRAINT "fk-user_tech-tech-id" FOREIGN KEY (technology_id) REFERENCES public.technology(id);
 P   ALTER TABLE ONLY public.user_technology DROP CONSTRAINT "fk-user_tech-tech-id";
       public          postgres    false    220    3215    218            �           2606    16851 $   user_technology fk-user_tech-user-id    FK CONSTRAINT     �   ALTER TABLE ONLY public.user_technology
    ADD CONSTRAINT "fk-user_tech-user-id" FOREIGN KEY (user_id) REFERENCES public."user"(id);
 P   ALTER TABLE ONLY public.user_technology DROP CONSTRAINT "fk-user_tech-user-id";
       public          postgres    false    216    3209    220            1      x������ � �      3      x������ � �      )   �   x���A
1E��0�4m��Ec(X������"#�����_,X(�%�:U�:��z@����SH�����|�O;��wi��E��?p�,����1����R�ǭ�F%��<*���?�g��%KΜ�Ƙ7��TD      -   �   x�̱j1�z�/k$��$�6�\��i��*M�@�|~����ɹ�|���N(�6̒��띓��o�����B#kcF�B�R�F�%r�����s�*�0/��e^�D"�bP.�y�(�:�-)U��a����0h�Q�x�*�}̘��P�C��	����,o���j��TrE���@ɥ0POj�����u������8�B�      +   �   x�5�K
�0@��Sx��L&���'�m7�LӊV��_Xy��3m�{{@W���,���I��^k�O��kY�����s��s/R���`��(��d�YK�#���d���$M����5�R� 3	� L�G��8���-�      /      x�3�4A.#N#0���� }     