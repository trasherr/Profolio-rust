PGDMP     3                     {         
   ProfolioDB    15.3    15.3 M    _           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                      false            `           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                      false            a           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                      false            b           1262    16398 
   ProfolioDB    DATABASE        CREATE DATABASE "ProfolioDB" WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'English_India.1252';
    DROP DATABASE "ProfolioDB";
                postgres    false            �            1259    25979    leagues    TABLE     �   CREATE TABLE public.leagues (
    id integer NOT NULL,
    title character varying NOT NULL,
    ctc_lower integer NOT NULL,
    ctc_upper integer NOT NULL,
    size integer NOT NULL
);
    DROP TABLE public.leagues;
       public         heap    postgres    false            �            1259    25978    leagues_id_seq    SEQUENCE     �   CREATE SEQUENCE public.leagues_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 %   DROP SEQUENCE public.leagues_id_seq;
       public          postgres    false    222            c           0    0    leagues_id_seq    SEQUENCE OWNED BY     A   ALTER SEQUENCE public.leagues_id_seq OWNED BY public.leagues.id;
          public          postgres    false    221            �            1259    25988    review    TABLE     �   CREATE TABLE public.review (
    id integer NOT NULL,
    user_id integer NOT NULL,
    caption_id integer NOT NULL,
    rating integer NOT NULL,
    text character varying
);
    DROP TABLE public.review;
       public         heap    postgres    false            �            1259    25987    review_id_seq    SEQUENCE     �   CREATE SEQUENCE public.review_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 $   DROP SEQUENCE public.review_id_seq;
       public          postgres    false    224            d           0    0    review_id_seq    SEQUENCE OWNED BY     ?   ALTER SEQUENCE public.review_id_seq OWNED BY public.review.id;
          public          postgres    false    223            �            1259    26002    review_slot    TABLE     �   CREATE TABLE public.review_slot (
    id integer NOT NULL,
    caption_id integer NOT NULL,
    user_id integer,
    uuid uuid NOT NULL,
    slot_time timestamp without time zone NOT NULL
);
    DROP TABLE public.review_slot;
       public         heap    postgres    false            �            1259    26001    review_slot_id_seq    SEQUENCE     �   CREATE SEQUENCE public.review_slot_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 )   DROP SEQUENCE public.review_slot_id_seq;
       public          postgres    false    226            e           0    0    review_slot_id_seq    SEQUENCE OWNED BY     I   ALTER SEQUENCE public.review_slot_id_seq OWNED BY public.review_slot.id;
          public          postgres    false    225            �            1259    26016    roadmap    TABLE     �   CREATE TABLE public.roadmap (
    id integer NOT NULL,
    uuid uuid NOT NULL,
    user_id integer NOT NULL,
    target_id integer NOT NULL,
    modified_at timestamp without time zone NOT NULL,
    created_at timestamp without time zone NOT NULL
);
    DROP TABLE public.roadmap;
       public         heap    postgres    false            �            1259    26015    roadmap_id_seq    SEQUENCE     �   CREATE SEQUENCE public.roadmap_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 %   DROP SEQUENCE public.roadmap_id_seq;
       public          postgres    false    228            f           0    0    roadmap_id_seq    SEQUENCE OWNED BY     A   ALTER SEQUENCE public.roadmap_id_seq OWNED BY public.roadmap.id;
          public          postgres    false    227            �            1259    26035    roadmap_user    TABLE     �   CREATE TABLE public.roadmap_user (
    id integer NOT NULL,
    level integer NOT NULL,
    user_id integer NOT NULL,
    roadmap_id integer NOT NULL
);
     DROP TABLE public.roadmap_user;
       public         heap    postgres    false            �            1259    26034    roadmap_user_id_seq    SEQUENCE     �   CREATE SEQUENCE public.roadmap_user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 *   DROP SEQUENCE public.roadmap_user_id_seq;
       public          postgres    false    230            g           0    0    roadmap_user_id_seq    SEQUENCE OWNED BY     K   ALTER SEQUENCE public.roadmap_user_id_seq OWNED BY public.roadmap_user.id;
          public          postgres    false    229            �            1259    25924    seaql_migrations    TABLE     q   CREATE TABLE public.seaql_migrations (
    version character varying NOT NULL,
    applied_at bigint NOT NULL
);
 $   DROP TABLE public.seaql_migrations;
       public         heap    postgres    false            �            1259    25949 
   technology    TABLE     �   CREATE TABLE public.technology (
    id integer NOT NULL,
    uuid uuid NOT NULL,
    title character varying NOT NULL,
    normalized_title character varying NOT NULL
);
    DROP TABLE public.technology;
       public         heap    postgres    false            �            1259    25948    technology_id_seq    SEQUENCE     �   CREATE SEQUENCE public.technology_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 (   DROP SEQUENCE public.technology_id_seq;
       public          postgres    false    218            h           0    0    technology_id_seq    SEQUENCE OWNED BY     G   ALTER SEQUENCE public.technology_id_seq OWNED BY public.technology.id;
          public          postgres    false    217            �            1259    25932    user    TABLE     R  CREATE TABLE public."user" (
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
    total_rating integer DEFAULT 0 NOT NULL,
    total_reviews integer DEFAULT 0 NOT NULL,
    company character varying,
    linkedin character varying,
    github character varying,
    others character varying
);
    DROP TABLE public."user";
       public         heap    postgres    false            �            1259    25931    user_id_seq    SEQUENCE     �   CREATE SEQUENCE public.user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 "   DROP SEQUENCE public.user_id_seq;
       public          postgres    false    216            i           0    0    user_id_seq    SEQUENCE OWNED BY     =   ALTER SEQUENCE public.user_id_seq OWNED BY public."user".id;
          public          postgres    false    215            �            1259    25962    user_technology    TABLE     �   CREATE TABLE public.user_technology (
    id integer NOT NULL,
    technology_id integer NOT NULL,
    user_id integer NOT NULL,
    score double precision NOT NULL
);
 #   DROP TABLE public.user_technology;
       public         heap    postgres    false            �            1259    25961    user_technology_id_seq    SEQUENCE     �   CREATE SEQUENCE public.user_technology_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 -   DROP SEQUENCE public.user_technology_id_seq;
       public          postgres    false    220            j           0    0    user_technology_id_seq    SEQUENCE OWNED BY     Q   ALTER SEQUENCE public.user_technology_id_seq OWNED BY public.user_technology.id;
          public          postgres    false    219            �           2604    25982 
   leagues id    DEFAULT     h   ALTER TABLE ONLY public.leagues ALTER COLUMN id SET DEFAULT nextval('public.leagues_id_seq'::regclass);
 9   ALTER TABLE public.leagues ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    222    221    222            �           2604    25991 	   review id    DEFAULT     f   ALTER TABLE ONLY public.review ALTER COLUMN id SET DEFAULT nextval('public.review_id_seq'::regclass);
 8   ALTER TABLE public.review ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    224    223    224            �           2604    26005    review_slot id    DEFAULT     p   ALTER TABLE ONLY public.review_slot ALTER COLUMN id SET DEFAULT nextval('public.review_slot_id_seq'::regclass);
 =   ALTER TABLE public.review_slot ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    225    226    226            �           2604    26019 
   roadmap id    DEFAULT     h   ALTER TABLE ONLY public.roadmap ALTER COLUMN id SET DEFAULT nextval('public.roadmap_id_seq'::regclass);
 9   ALTER TABLE public.roadmap ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    228    227    228            �           2604    26038    roadmap_user id    DEFAULT     r   ALTER TABLE ONLY public.roadmap_user ALTER COLUMN id SET DEFAULT nextval('public.roadmap_user_id_seq'::regclass);
 >   ALTER TABLE public.roadmap_user ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    230    229    230            �           2604    25952    technology id    DEFAULT     n   ALTER TABLE ONLY public.technology ALTER COLUMN id SET DEFAULT nextval('public.technology_id_seq'::regclass);
 <   ALTER TABLE public.technology ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    218    217    218            �           2604    25935    user id    DEFAULT     d   ALTER TABLE ONLY public."user" ALTER COLUMN id SET DEFAULT nextval('public.user_id_seq'::regclass);
 8   ALTER TABLE public."user" ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    216    215    216            �           2604    25965    user_technology id    DEFAULT     x   ALTER TABLE ONLY public.user_technology ALTER COLUMN id SET DEFAULT nextval('public.user_technology_id_seq'::regclass);
 A   ALTER TABLE public.user_technology ALTER COLUMN id DROP DEFAULT;
       public          postgres    false    220    219    220            T          0    25979    leagues 
   TABLE DATA           H   COPY public.leagues (id, title, ctc_lower, ctc_upper, size) FROM stdin;
    public          postgres    false    222   	Z       V          0    25988    review 
   TABLE DATA           G   COPY public.review (id, user_id, caption_id, rating, text) FROM stdin;
    public          postgres    false    224   �Z       X          0    26002    review_slot 
   TABLE DATA           O   COPY public.review_slot (id, caption_id, user_id, uuid, slot_time) FROM stdin;
    public          postgres    false    226   �Z       Z          0    26016    roadmap 
   TABLE DATA           X   COPY public.roadmap (id, uuid, user_id, target_id, modified_at, created_at) FROM stdin;
    public          postgres    false    228   [       \          0    26035    roadmap_user 
   TABLE DATA           F   COPY public.roadmap_user (id, level, user_id, roadmap_id) FROM stdin;
    public          postgres    false    230   l[       L          0    25924    seaql_migrations 
   TABLE DATA           ?   COPY public.seaql_migrations (version, applied_at) FROM stdin;
    public          postgres    false    214   �[       P          0    25949 
   technology 
   TABLE DATA           G   COPY public.technology (id, uuid, title, normalized_title) FROM stdin;
    public          postgres    false    218   �\       N          0    25932    user 
   TABLE DATA           �   COPY public."user" (id, uuid, name, email, password, phone, phone_code, profession, ctc, experience, total_rating, total_reviews, company, linkedin, github, others) FROM stdin;
    public          postgres    false    216   b]       R          0    25962    user_technology 
   TABLE DATA           L   COPY public.user_technology (id, technology_id, user_id, score) FROM stdin;
    public          postgres    false    220   �       k           0    0    leagues_id_seq    SEQUENCE SET     <   SELECT pg_catalog.setval('public.leagues_id_seq', 7, true);
          public          postgres    false    221            l           0    0    review_id_seq    SEQUENCE SET     <   SELECT pg_catalog.setval('public.review_id_seq', 1, false);
          public          postgres    false    223            m           0    0    review_slot_id_seq    SEQUENCE SET     @   SELECT pg_catalog.setval('public.review_slot_id_seq', 1, true);
          public          postgres    false    225            n           0    0    roadmap_id_seq    SEQUENCE SET     <   SELECT pg_catalog.setval('public.roadmap_id_seq', 2, true);
          public          postgres    false    227            o           0    0    roadmap_user_id_seq    SEQUENCE SET     B   SELECT pg_catalog.setval('public.roadmap_user_id_seq', 10, true);
          public          postgres    false    229            p           0    0    technology_id_seq    SEQUENCE SET     ?   SELECT pg_catalog.setval('public.technology_id_seq', 5, true);
          public          postgres    false    217            q           0    0    user_id_seq    SEQUENCE SET     ;   SELECT pg_catalog.setval('public.user_id_seq', 122, true);
          public          postgres    false    215            r           0    0    user_technology_id_seq    SEQUENCE SET     E   SELECT pg_catalog.setval('public.user_technology_id_seq', 1, false);
          public          postgres    false    219            �           2606    25986    leagues leagues_pkey 
   CONSTRAINT     R   ALTER TABLE ONLY public.leagues
    ADD CONSTRAINT leagues_pkey PRIMARY KEY (id);
 >   ALTER TABLE ONLY public.leagues DROP CONSTRAINT leagues_pkey;
       public            postgres    false    222            �           2606    25995    review review_pkey 
   CONSTRAINT     P   ALTER TABLE ONLY public.review
    ADD CONSTRAINT review_pkey PRIMARY KEY (id);
 <   ALTER TABLE ONLY public.review DROP CONSTRAINT review_pkey;
       public            postgres    false    224            �           2606    26007    review_slot review_slot_pkey 
   CONSTRAINT     Z   ALTER TABLE ONLY public.review_slot
    ADD CONSTRAINT review_slot_pkey PRIMARY KEY (id);
 F   ALTER TABLE ONLY public.review_slot DROP CONSTRAINT review_slot_pkey;
       public            postgres    false    226            �           2606    26009     review_slot review_slot_uuid_key 
   CONSTRAINT     [   ALTER TABLE ONLY public.review_slot
    ADD CONSTRAINT review_slot_uuid_key UNIQUE (uuid);
 J   ALTER TABLE ONLY public.review_slot DROP CONSTRAINT review_slot_uuid_key;
       public            postgres    false    226            �           2606    26021    roadmap roadmap_pkey 
   CONSTRAINT     R   ALTER TABLE ONLY public.roadmap
    ADD CONSTRAINT roadmap_pkey PRIMARY KEY (id);
 >   ALTER TABLE ONLY public.roadmap DROP CONSTRAINT roadmap_pkey;
       public            postgres    false    228            �           2606    26040    roadmap_user roadmap_user_pkey 
   CONSTRAINT     \   ALTER TABLE ONLY public.roadmap_user
    ADD CONSTRAINT roadmap_user_pkey PRIMARY KEY (id);
 H   ALTER TABLE ONLY public.roadmap_user DROP CONSTRAINT roadmap_user_pkey;
       public            postgres    false    230            �           2606    26023    roadmap roadmap_uuid_key 
   CONSTRAINT     S   ALTER TABLE ONLY public.roadmap
    ADD CONSTRAINT roadmap_uuid_key UNIQUE (uuid);
 B   ALTER TABLE ONLY public.roadmap DROP CONSTRAINT roadmap_uuid_key;
       public            postgres    false    228            �           2606    25930 &   seaql_migrations seaql_migrations_pkey 
   CONSTRAINT     i   ALTER TABLE ONLY public.seaql_migrations
    ADD CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version);
 P   ALTER TABLE ONLY public.seaql_migrations DROP CONSTRAINT seaql_migrations_pkey;
       public            postgres    false    214            �           2606    25960 *   technology technology_normalized_title_key 
   CONSTRAINT     q   ALTER TABLE ONLY public.technology
    ADD CONSTRAINT technology_normalized_title_key UNIQUE (normalized_title);
 T   ALTER TABLE ONLY public.technology DROP CONSTRAINT technology_normalized_title_key;
       public            postgres    false    218            �           2606    25956    technology technology_pkey 
   CONSTRAINT     X   ALTER TABLE ONLY public.technology
    ADD CONSTRAINT technology_pkey PRIMARY KEY (id);
 D   ALTER TABLE ONLY public.technology DROP CONSTRAINT technology_pkey;
       public            postgres    false    218            �           2606    25958    technology technology_uuid_key 
   CONSTRAINT     Y   ALTER TABLE ONLY public.technology
    ADD CONSTRAINT technology_uuid_key UNIQUE (uuid);
 H   ALTER TABLE ONLY public.technology DROP CONSTRAINT technology_uuid_key;
       public            postgres    false    218            �           2606    25947    user user_email_key 
   CONSTRAINT     Q   ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_email_key UNIQUE (email);
 ?   ALTER TABLE ONLY public."user" DROP CONSTRAINT user_email_key;
       public            postgres    false    216            �           2606    25943    user user_pkey 
   CONSTRAINT     N   ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);
 :   ALTER TABLE ONLY public."user" DROP CONSTRAINT user_pkey;
       public            postgres    false    216            �           2606    25967 $   user_technology user_technology_pkey 
   CONSTRAINT     b   ALTER TABLE ONLY public.user_technology
    ADD CONSTRAINT user_technology_pkey PRIMARY KEY (id);
 N   ALTER TABLE ONLY public.user_technology DROP CONSTRAINT user_technology_pkey;
       public            postgres    false    220            �           2606    25945    user user_uuid_key 
   CONSTRAINT     O   ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_uuid_key UNIQUE (uuid);
 >   ALTER TABLE ONLY public."user" DROP CONSTRAINT user_uuid_key;
       public            postgres    false    216            �           2606    26046 +   roadmap_user fk-Roadmap-roadmapuser-user-id    FK CONSTRAINT     �   ALTER TABLE ONLY public.roadmap_user
    ADD CONSTRAINT "fk-Roadmap-roadmapuser-user-id" FOREIGN KEY (roadmap_id) REFERENCES public.roadmap(id);
 W   ALTER TABLE ONLY public.roadmap_user DROP CONSTRAINT "fk-Roadmap-roadmapuser-user-id";
       public          postgres    false    230    228    3249            �           2606    26041 #   roadmap_user fk-RoadmapUser-user-id    FK CONSTRAINT     �   ALTER TABLE ONLY public.roadmap_user
    ADD CONSTRAINT "fk-RoadmapUser-user-id" FOREIGN KEY (user_id) REFERENCES public."user"(id);
 O   ALTER TABLE ONLY public.roadmap_user DROP CONSTRAINT "fk-RoadmapUser-user-id";
       public          postgres    false    216    230    3229            �           2606    25996 "   review fk-review_caption-target-id    FK CONSTRAINT     �   ALTER TABLE ONLY public.review
    ADD CONSTRAINT "fk-review_caption-target-id" FOREIGN KEY (caption_id) REFERENCES public."user"(id);
 N   ALTER TABLE ONLY public.review DROP CONSTRAINT "fk-review_caption-target-id";
       public          postgres    false    3229    216    224            �           2606    26010 ,   review_slot fk-review_slot_caption-target-id    FK CONSTRAINT     �   ALTER TABLE ONLY public.review_slot
    ADD CONSTRAINT "fk-review_slot_caption-target-id" FOREIGN KEY (caption_id) REFERENCES public."user"(id);
 X   ALTER TABLE ONLY public.review_slot DROP CONSTRAINT "fk-review_slot_caption-target-id";
       public          postgres    false    3229    216    226            �           2606    26029    roadmap fk-roadmap-target-id    FK CONSTRAINT     �   ALTER TABLE ONLY public.roadmap
    ADD CONSTRAINT "fk-roadmap-target-id" FOREIGN KEY (target_id) REFERENCES public."user"(id);
 H   ALTER TABLE ONLY public.roadmap DROP CONSTRAINT "fk-roadmap-target-id";
       public          postgres    false    228    216    3229            �           2606    26024    roadmap fk-roadmap-user-id    FK CONSTRAINT     |   ALTER TABLE ONLY public.roadmap
    ADD CONSTRAINT "fk-roadmap-user-id" FOREIGN KEY (user_id) REFERENCES public."user"(id);
 F   ALTER TABLE ONLY public.roadmap DROP CONSTRAINT "fk-roadmap-user-id";
       public          postgres    false    3229    216    228            �           2606    25968 $   user_technology fk-user_tech-tech-id    FK CONSTRAINT     �   ALTER TABLE ONLY public.user_technology
    ADD CONSTRAINT "fk-user_tech-tech-id" FOREIGN KEY (technology_id) REFERENCES public.technology(id);
 P   ALTER TABLE ONLY public.user_technology DROP CONSTRAINT "fk-user_tech-tech-id";
       public          postgres    false    3235    220    218            �           2606    25973 $   user_technology fk-user_tech-user-id    FK CONSTRAINT     �   ALTER TABLE ONLY public.user_technology
    ADD CONSTRAINT "fk-user_tech-user-id" FOREIGN KEY (user_id) REFERENCES public."user"(id);
 P   ALTER TABLE ONLY public.user_technology DROP CONSTRAINT "fk-user_tech-user-id";
       public          postgres    false    3229    216    220            T   �   x�3�tM,*��4�4�42�2���I,r�@\cΐ��ļ�Ҝ�\������	�ofNv�Bxb%T̔˔�1/�(?75%$bh�ih�e�铘�����02��sd'�?j��ZRZ600 \1z\\\ ��#T      V      x������ � �      X   E   x����0 �7Ta8�hj�E��Kp�Y��@�w���7'�h#��P%�n��HPV#��v/��{"�;}x      Z   M   x�uɱ�  ���x�(�,6�\�!��W����~�lVRF�xܽ�
��ډ�T/�ż8������"~��&      \   @   x�%���0Cѳ=�IJ�.�??�!ʓ���R8��A�*0<���;N�~��ɸ��D�e��
�      L   �   x�}��� ��q1�|�ʽ�1ҳj��@���L��'<���� L]��n倅��N�U� CA��d^���}��m=F	GJ$���#��˸���#cEf�A7y��� ��m?���v\���7���QB��;n���`�rQ�����������ug6�1��m�9�NG���	�l      P   �   x��;k1 �Y�/
�-�ƃ�Bh�pI�C�du������o�b.Y�)�ZE�2P�g̽I������}��v{q�
��-�����&d(ܽ�B�|���c��0�O��iq��B��H@^[�V{�6TR�#*8��s�:�@����?=*֪�6Z7�c��?:�w�_����% ��V�8|IȜ=�b+�Z�Jy�e�?`�]�������C/      N      x�}��r[ױ���~
�m�b���p�-۱cY�#;q%�W��( `�~��k���qYJD�gw����2S]��!��٢f�t��Z�\�hY������ݺ\|��nV�i��-�ޯ~�վ�W��wW���˺��b�mx�R��<�6ݫX��!D5���vg��/E���j�KOEy5i���:���d���w׫��v���&�4)�y�W}wQ6�����v�?���?�/�K�I�c�ev=�y.��,��)������]���z?���^��U����G�,>��cj9�XC�#x�L���RS���Q�����X�U0��%�1Z�S6�2�xi��b���l.v�v{��~;9�s~.����⯛��7-!u��2ה��JsV��F)�t��P��������ŋ'�+�K__ݮ��w��ߔ��h�_tH���#fs)ù��j�X�y�O��*��N�egB�_\���gV��/lԳ���e�]o߬n��Γ>��ժ��)�OL�S.K����-��-!�ٹe֮����E�����v���_��s�L�]��ͻ�n�GY��Yo.�|�����Dj$f��tU,��Rb0M��|1Yu��&՜9I|k�1�۾ٕ�B�O��1����S���ay6���oK�a��~.ֵ9����c�Ӌ]�_����Tn��U������n���7�cFr�,&�Ʊ/K��WW��)cn���tKQe�l���Pv�.��1A�:�9�<Gc�5V�>Vo��X&��V��;����/?Z'3ڢ�Q��]�z.1�y!lI���	�����?��xg��~�7�����ӄN��G]�&Z[)ޖ���u\r�ۢ��K�݄MjQ�A����w�I��3���H���d����Zrw{x�ц4�rv6���%�:/��|������m-������j��t��d�f%X�>KWt�R�d��m�p���TGԩ)�ɎHR�ԳZ�E�
��'k�%��������ݦ�]��ź�?��)������>�ӒC+������s!kg���+!m�/�}_.�ؼ=l�j�f�)�7��?��w�[�������v@%�Z��f�p�ђ�}�3����q(�[_�qK�vV��6x/�nWД�:%����ܐ(oȕ'��k�u}qF�]�@9sZjN�4���n����M��e��n��u/W<�Jj��T	�V�q|���{
�U\,�d���u���$)2�eX��Mɇ˜�%�;U��zU��v~�I���A_n����h��lK�ϑ��=��I�1�������Cd��=qV�߾]M��j��<��b���1�;��Ko��Bv;">!���G��:���)�ј4Ab��y6��Q/v�����L�^��f�A�:g|�%�ޫ������\��ל���oZY��~ۮ����}�D.�?��xLeJ��{=6�Z�©@$�z�uq��m��T�+`	Pd9�5�z����x��v����
��^��"��/u����*D; �`�j��~�[P�/~.���m�}w}U�#�PkR
�/�0lHi���ue�Zn`B�5�����z�C4ج�!����7��q�QILZm��wT�S�����v�!�n|�sC	7Ag��^�.�ȡDK�=��
�݀۫E"�$:�" ��YRcإC��R]H�r���y��f[�CW��D�e�ڳLƫK��e��>��� l/׫z����v������kMY�}]�b�o&������D��;U�����sd��G!W��t�AК���(2:��	6/���0���:D�����[���S����}�yyX�ۣE���9GQg��+x e�:B ���J����D�,�I<U��w&u�A�8���讛��.������u�h��1����h�0@P+����T1�E�˟��z����h��N$���ߛ$����u�`Z�J�����,j�^�)�F���<A7Ui����G���Gag/U��\�=��A(j�G��:R�{�K*���;'��]��.]G�k�
�=·��vlw�CҔL�����J�Śfh��;,�� `S�o8\�b{[���S��@�1CP�g�!4d�+rpQ�3L�f1���S�S -��:�Di;�����.zRP�Xs*@��� |�;��oa+o���<u��"b*:�>�\��R$�# b����40���(���Uy�JM�叚a/�kR'S� �,ڦ���6"�(y�(�Z����g�uB��sQ,{�N��=���챴<c��J$�a�K�ЄJN�[*F�,�x=��j���ಁ-��b	:���3Gi�� �(� �(�F�5�L�.eӲm���Kr�#� i�}2O��K�Ww{0�ûw����[ V
j��ġ��+����S��^�wG~;��77���pn5d������A0�*����@����S�s�ji)�;'zr�����y0�=�Dș�����)��$6Z�mDlT�
P�i��T9��N�� �����F��4�<E�I�1�"���!	�sAD�$�TSc����C�E~j��e�d�E ���w	��Ժ=l��P�poNzJ�r{�z²���2�twe��[���M������uL�����ؾ���Ǭi��q��q�+=��(H�k�I�{�#jZ�:��!�� H��q��k<�)*�����0��'��k�n�EJN+8��:��������@�ŏ��[��&�j�'���z�^xl����;�%N�C�L�o$ȾVW��p2K)$�k���i�O�9"4��$~@{�B�8�O�R�uD�iiHž�e�HԗL����"�vRk���v���n��WG��:�Fq�C���,�g�"D+T�/���0�H
*J-�'F�����:	��QoŒڛ. }
�����fs�=���n��?�lar�� r�3��1C4U�m��ˁ-#��ƈ�XX�=�6W��q���;h��A l�ǂB�@+q�p܁�
#w��jk1q�xנ)�d��Q3y蟑AIO������~4����`�.�Q	��q�< Ә���Q��Y�����b����⵨U麑J�|�z�J ��*�-�o�}�gQ�
�`���6Ȣ�ѷ��=6�(>"W�Ѱ}/���f������~�w�?�45
�K/�q�+ј=2��c�#?���G?�L7����_���.,�e��@ |;T�V������;�������Du5��1,�K��e�=�{�/� )I���z�$2y*$�-*,h���9̈́	�U����UY���O���t�]����˾<&s~@9lL�� �!���Ԓ�i)�e�#�xBN�vx�0TGiha����u�&;uD�7�U=�9�c5���ѿ��������� �|����h���;���Е��B��K��-�=����Zm���v�dM��c4u��99���1#��y��d��TU|Z�kw�d��,*3<c̃��i�~�&�8p;���@�2�/Sf�(	�@G��4*�>�P�@��-��/`ڛ����ޮn���3Ӫ��"�`6Pepr�襂�3�0L���b(�$MҡQ^���誀��%㐯�+��?[�~��oˮܗ��n�i�Κ�PITq�g%�C,z^�(�iH�n�/$���P�������|"���9�X�����SIJ:�q.U��Z���$��%
��}��]"^. �\H�s�W�ݻ�AGh���� �>�f�(��R
Lu��+�������=�ؘ����qL���i�"��?ʩޠ9xP��\u�9D�5a:}~~�������=o����w��'"�:Q#��h����p#�Q\\��H%���z������B���qSA��-��0��ɸ@q����HK��H��{i�*j��V�8T��Bx�0�h���j���aT"��<|2��L���E���� AF3���H�����6���TnVwX�X�O~/�Є-z�ب4?�d1��Q�H'^+،��	]����!�`����I�'�>���'<�f{ןk������"�E�P����X�c��V�����#ݫR1�j�k���x�|t���^�����RfR">�ۏ��g����x�a��	*̽    K#Mt�����ס�s��)��5`��v���k�.2+��C�֫�bD��q4k����(�6 �#���80i�m��|�)~���`��ݼ>{+���T�ܡ_ĦX�dHХ��<���)E�5VG���- >#����qG} /B�a�CJ��TmjN��e�`R�-�aP��F.[�$l�$l�p0���e�j�ݝH�+�Q��V	���1��Qx0
z�6ኂ����Y~_�d�1�A����?�s.M�I:��KU-�ymd< e�mXBX��p�!�@�&M2�ե+��^ڶDP�:�r�8��?9�$C��.>aFH4	��g8!~6 �k+�L?P`j��������R"R�&!�h���R2�c+2$2�������2��.�s�ԨO��~���7�ny�}S��8HM�<��fc��Q�ֳZ��eab�7��oe���g�x`M�Xm��[3�41E�H2=!�a�� ��2!c+�	٬���aД�8��BO���gi�=�3�~�������S(":#���܃�8&ÅCUsBC)N4�Ҧ/�wuU.�Y-�5o �6����q��U�[%�߈����Z| ��a�5�7�kB��˗R�u���:Hp/\^�v����}��Lp:3�a�H��!��f�RC	vW�������~�0�ޭ����p#��2^a3"�`UT��#,a�F�m�`iƟ_eA��i�+B�c2͎>�.�����g�vп��y�O�]���a�S6��B�8.�yN2G�0��2�R'�O�}K%\�'����?���=�چ����40+�ģ�T��ݲ��P����#t�C�D7}��BE?kҚ�9���	��ק��kY��t��@�!|���,y q�V�K	ˀ�.p��������uٿ�`�۾���R{w��iC�G�!��H(�2j�B���h���ȕ� �QK�J���du�o�CJP��N�+w��=�O)���<>ũ����ώ|���(C�sV0�Y&�3:;��8�QK�/������զ\�Uݟ�d�a�xz�<�: ������=I9ؑ�NA�A����Bj(��슒�l��]�ǵ���]&r������nH��}� �]�2NF3���L1VߒR3�7�cKE��}�]�Nn��ݓ�H���PD%�H�$9�%}���G�B�x��,	�-0,�|���'t3<��_��ᢋ� b@��B�A��SCZ׵��������ӂ��WZ(���U_#�7��H�V@I0`�i
j�z��� F�!��7(����q%�p��LF�?k�l�Z�,��q�c`�'� �§Cr)�]�V�.͢>��lF=�޼�^^��j-κY���Xo��'����K���(�7⨄]�B��P%� ؈�B�:y�/L��Uj���>�����Ͻ=��G]�t+VT357iW�lz�����J�W�8j�����j�ޯ�'S�Ջ���f�,�loΈ��k(�PTzm �SK�g��L2��#�8��^�B,�j�WS��P?Ƭ��فPs�(g_� �����ۋ���g��k���T*Չ��P�A��	9�Ck
V�� �2�q��v���%7ٶ�S5�Ҫ|	���=ݛChP[>��h�~�H�(N�3��Y��!l������Q�����}_n	W0\f���� ˍ�,���q��.�|���d,�e��p|�t�2G�h9hvmXh�b4�fN@�HC��n����N �����o&���
�Z�6��AJS?�.��PX�N(����Ԑ_��ޔ��*'8l��qD�Z�B�ʋ��_�C1SQ�	ONu�V���O �I6�H�*>�ye��Y^z� j��v�-	�TR�-��~����^��ŷ�w���[���W���P �"�G���]�.#d�"O��V��F����8�nֈPoN �ho+�}o��9��=��~�����|��u���f�qV����(b���߬��Ɍ������;4�+I仮�?��� ��~�8-��U6E-l�LE��p<5}r���ǐ2���p�/��ԭ0�S�p��t��Op�Xz���#��ϔ�
A��Үh�/��������[��|�.�OG��
���YƔ��P/�;�iQ[·c���~^��
�g9.��pٙ |4���ﴽ���x8�,v�b���q#p�u@G0�R��[�~>H��r@�L�!�`?����)�8d�I�M��l�Ĥ��G�uI��dHe4O�5���4���~�Rz���Æ��*�����F	SX@/�����豢�'9�<��٭�m�Lo��������<����](nq�Lw��I���Q<��I��A����R2�I�H��,{�� �xO�C¼*w2<
�^���h�#����l�����($��*�B�
w���4��j7ػ���j]��n��=�6�e��6�#"38Z֡BA�m(�L�"=S��!���$]K8���%~7��v��	n*��v�Ǳ�iB��~2�yJMD)�<g���q�s���Q���d�(DaG2��ڬnW�$���4��T��o����C)"|�H1��ˢ jH��,���� B��J�ɝ��?�U�-���l'�t0�j8'ف�&�qn��7�k�D���~+s9��������^{Y��?p����(���������Q�5*�A���Yg�b�y�?�:ٶR�.�u���}?�g��c�g�Ce�E�	�P�C�<BoK��z�j��%�Ejl������_�ڼ��� �KܫI��H> V��� �ճ��!�-�6��/���ug����w��S�\lΪ.��:��}�!��&R�ӷ�&H�r�G��n���'�v��f���%(�%�`���Y{n-זf,Om���MH��!��5Ͱ����f��ì�SGة�dp��)i��4PG�C���q�s�c��*˹.��,�jѢ�/.���P.W�B�m#�>�~5�UU���J�:�z����7?����]��u��u��)opj��`�:�y�騳��7p�\��B8e7�غ�������<�m�`F���&�A<5����P��%Jou���1���|��%=rr�_�8�R=ޝ��S��v����{b��,%�3Y&�F��K�q^x���1��"[���o��f*�����o�8ʠ^�P�b�#A�A���g�?r�T�JxU��n����&�m�P?��r�`�'l�߷�.7�:vO�,Lƒ���(!�j++�q/�&1q?�8�e�{��ʉ���5�Pd;gB���V�vj.|c@�*�%$��`���� ��6�F��Eg��'��3U��پp ���hW{�]�!L����"	q$�K2�"�d>����c�)��k0.`�T�ʏ�ZfAĴ��� i���V5 @_�U�QX�ebv�ϴ�N.��=�ӄ!PcjN3��i(�Мe��eMC��q���k$��4�}��d��,:���-X%:d��XةAR��ᕎ CB�wb~�e*]�S��O���y��z;�����!�X�6�/���f��)bn��&-���]y����o��^�ö��ukjψ��(̊��mȦ��Vm`��Ҽ��
�-Jsjx�((n:)Y�OG澷�=����I�� A��3p��en�����^m��cķw�+U��A���v_Q���|���m��4���/K�{�B\����x�U�J�3K������w�N��ǵ�'�$��m����UޛYn,ζG>Z����A�y���7��w��H<U�R)�%� ��ٴ(�̷��l�(
���,�:XJ�v7��N�&A�L��]�zXաI_F3��Q-$r`�i^�C.t���" �M�֫��8�����%���AR��@_�E>_!�NQul�X�r��D��� ���N*'&�l�Y�<BrO�u�p�D.b��b'����o�J/�1d�����Ӡx�J�z�_�[�h�yD{�'�v�b��eo �Џ��B]�F���kH8=�`lr��g����HuZ�=ۑ8د8]B    ���� �:�,_ k4DsF'˅�fgi�Ω��M�V��X�
��*]���"�����Nr��JS�`"�kUrˤ׎J\N�M�dem�k�NUY� �d�����:e�/q�%��4F����ö�A~������AB���m)�ٖ\c�A6YЪ�����آ��?tB�����6l���UJ6h�FuI��G�5d�U�R*�Y�C9����ר�rCg/CSȝ��-�������r��Ep����7C�eϨ�%83�[����K!�Dʽy��W�Ǖ��<��ay�����6��a���[�|@g�9�PL��t��Vf
�l��a���!�d}|���NO0<N�u���P��N�x��CC�b�~~{'���X�������g}@u���\{�A]���ɣ�Co�����C��U��-�rDD5�t)� ��i�m+9+���,�R���<�N���ţ��$ˇ��w���ܛ��j�o��s��=��xga�%��������u���<Աh�B�,r��%4���b����2w����e�ω��k����d�%'1����n�ɹ�)�ޡ� dWAva2�0S��0t�^��ʟe�~���<7+��>���'w4L�׈���	e�[!��w(V3��<�M&�H�zKK?f�H�$���K?N�������â(d��9v'ŗvO�J/ы��[�Q�y�^J����m������m5�}��Ի�֘��+��9I[!�F+���[ҨG_%u[F	Ye[�dΗJ�KO��O�ć�s��'&hjE�z�1S�%{A�J+'tS�/��&����5(ݏ�S����e`4��2��t�"��JH������Bޔ�����06��K���UZ�;E�OI���I��v��O����ɽ!-����s˕D���ÿ?m*��.c������|A猑jqǂcJ���eIqT�W.�W�x�?[���t�;Z��G�?:���?t��+��O}��
�Es�7(d��Zd���Z�ɀtj*Nߖ?�����M�֮�MϪ�s$�c�ԑ9Hf�"���U����`����#�,�Y&+�S��{LO�	��q�?�~��%�`zu�rnUx#!�L��
��2}���/.;L�eWV�V���
ia3��.h&��I+�v!��[��${q�pl"%6d�f��������*�/��S�	���1r���E�.ku���G��q\P��/����W�[���E�ݑ�'�v��;�]8�T�1ȁ({��3d-D�_'Y�=�pT+��&r}N�䘜�!����r{��߃{��_=e2(�A1D9G�@�kf�ck�����E�O�ӏ���z���Y�ܟ�G��(7�ʲ z�R �>)+��F��\[|�PR+E��{4�⌀1%r���*��JT?t����&B;RqA9�o�gٕ���v6KC�-�d���P���oV{�/}���))������#�J���̦ .�W���!��	����A�v0m8��)8P+�KPT?M��[o_��w�+ߖ��Ei�c�,D��w��E���%F|�`�0�������Y��8��^`bF~P%���	�,�q>�����d	E�?&��@��l�Q~�_|�[����!b)Op�������
۝�E[��jԖ��m��yv7m�Ч�P	�pXZ� xӹ����Pc�p�v�=?�,룶�
2p!w�dӥ����T�l@%H�������y�3YM]�Lq+3��ɫY�V��ؔe�J�1��X)O��Ѽ}v�]�e@�H������ i�r���9nW��6>I�זF]3 <���y��J�?,������~�^��\h��S�-�a�CU*�� ��l*qz���=�pY���{r[^��J#����;u���,�(i�Հg��cfrZЂ-$�+)������_$-�%O��/"���r�J����r�d��x�{Yf
��=0���fe�k�AG�H_��v�*&�S+�m7F��p�Ty�F�CJ��0T�1FA�r�Y7�OjȜ.�<��N��",�a�U�3��>�0��j�,c�'�S;v�����/
�Ͽg�������++Y��2��v���'KE-�Խ�贏�.�
�r�-"�K��$���0Q��,���933
 ��eV��B��U懆�C��3�+�t��6�< �"���s�u��Z�)S_�+F���W��� ��M������q�qn�N$Y���A�a^,�G����a=�	; c�>t[`fI}~a���p�_�f]�>�4?�����ɐ*H�"�`c�7�T𰑞�t�#N��8��7�ǯ�8�]A��9�w�%�р	��K(�H��z����	�UU�M�xI�^R��V�}x�ė�������ynWk��t #��r��Ȉu	�<�iw �j�xXΜʡ�� |rk�S�%��M�U5���KrF���ie����P� `5 /04��IM���yu�ݬ��)�&�c��� �����H�,��\�u���W�=<L���'9���5�*�F�E'����ZV�P����eT[<������#o��,Q�0
)rN��C�I�Z���?>��4�g!�s���Qxz�u%�ź���ű<�j�����=6�<Oڡf���H=�jYd[��.)jԁ+Q�V��
��5=�!������r��wg��Do�WNEgu���O0-����!#!0 *�=��)HXA��:���˾�O�޶���������0�Ш�|�+Y*B�%ԣ�k[�dD���ZSR�إkEXbQ�s������?���������;ɀg_�����~&T���y�����Z^:"���U����/������l�wM!��̧���Z
�
_���n -��ܡ���9�k��jʐN����p!�U��v?K|o�~��#%�8$eF�ˁ�y�k���Io�!V��B�����M��onoe�[��NΓ+[��ͦ���0�9�.m�0�hI#Fg���SG� D�y��e�2r����,��w��;���B+35i�;�q�Kw�i�g�g!��/ˮ����ؔj}����զ?�I���7_���aКF_���=�'�� �	��*���9/mqsHP�p �����~��a��7�W,�UW������ߴ���?���'���O�(��W�O�3��N!Z���;�c%��a=`�!��י��������K�K�P'�ݘ�ȋ��Pm���~��I�����7)��7�JJt�d\�u:~y���Woo��CTe@��$/0���G�'�˚�H	3������7�����Nn�-��Ucs���������7��.�:M����e�R�]��NQ�C�� /gif�m_�JC@���R�ض.��w�����~����7��I�B(R��sO�?�����"w�Q��'t�4!]�J-$�,}�a�.C.����,�V�C�_Y���/���_��󔵄ӛ���s�V_�;��[D{"BQ�����"�|q{]�ķ�VN��*����f����a��KT7k���N/u�������j4�<�$G:�Y����d�@b��]ٵ�� ��������e�ۧ���Ku�ə?�h×Z�,�+���$<�83��)*���i��⛾��͡l�}y4|<hZ}�����my!G���Z���Rڡ��y�����@��D :Ƶ����d86��2��!W��,1ׇ������� M�C�~�N8�K�s��˽��*�o��G
�y8��.�� �\����������z�"f���0�?{?U�\��v돯�B��j��4�Ls~L/�v��M$���j��|��o��������~y\�J��n�iu�~~���n��u���k����c����qB�_n�.Ý��ew����GG|���a�~�q�k*��m⟢�NtaK���,G�:�lլ����tPdgW����q?������ۗ�˿Ȼ|\�x��iB����e�S��f�,�d`���~u$��6��z�9����)��Փ7D����cl�1�=�E���ݑ'��d��E��{z�,�(<U�]ޤ��>�&�� �  �V�i�e�Ӯ��]_����c%[��_����e��$�5�uи?{�&?4��D6��3��<.Aƛ^�{���T���W������b�v��������_^��������0ʱ�_������|K��Z��4�'=Cܢ��C��U?��\��ny�����7���ÿ~��-_��n6J^4���.7}x��t����
�ϚGu��Ӯ��W���ڼ��ޝ�������L�	h�U�rP�5߀�
bYE�˕�����G�^�)'4��	�e��R�8��xF��gf�1*5��yY���s�-�^��W����ы���,�N�t���fy{���*���F��(D�,��|D��[q�31��GL;���`�����V3�u���F.tl�����]���������      R      x������ � �     