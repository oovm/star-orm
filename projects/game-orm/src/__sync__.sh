sea generate entity -u postgresql://localhost:5432/postgres \
    --compact-format --with-serde both --with-copy-enums \
    --max-connections 5 --lib