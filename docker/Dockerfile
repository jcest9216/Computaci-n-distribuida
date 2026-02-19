FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    wireguard \
    iproute2 \
    iputils-ping \
    nano \
    curl \
    build-essential \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p /etc/wireguard

RUN wg genkey | tee /etc/wireguard/private.key | wg pubkey > /etc/wireguard/public.key

RUN echo "[Interface]" > /etc/wireguard/wg0.conf && \
    echo "Address = SUSTITUIR_POR_IP" >> /etc/wireguard/wg0.conf && \
    echo "PrivateKey = $(cat /etc/wireguard/private.key)" >> /etc/wireguard/wg0.conf && \
    echo "" >> /etc/wireguard/wg0.conf && \
    echo "#SERVER" >> /etc/wireguard/wg0.conf && \
    echo "[Peer]" >> /etc/wireguard/wg0.conf && \
    echo "PublicKey = iQRu1M/ijoX5KsnjhCvxOrz9VtW72dBXXtKkXzoscEo=" >> /etc/wireguard/wg0.conf && \
    echo "Endpoint = SUSTITUIR_POR_IP_PUBLICA_DEL_SERVER:51820" >> /etc/wireguard/wg0.conf && \
    echo "AllowedIPs = 10.0.0.0/24" >> /etc/wireguard/wg0.conf && \
    echo "PersistentKeepalive = 25" >> /etc/wireguard/wg0.conf

CMD ["sleep", "infinity"]
