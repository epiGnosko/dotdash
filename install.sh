mkdir -p /usr/local/lib/python3.x/

rm -rf /usr/local/lib/python3.x/dotdash

cp -r dotdash/ /usr/local/lib/python3.x/dotdash/

ln -sf /usr/local/lib/python3.x/dotdash/main.py /usr/local/bin/dotdash

chmod +x /usr/local/lib/python3.x/dotdash/main.py
