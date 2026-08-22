#!/usr/bin/env bash

surreal start \
    --user admin \
    --pass admin \
    rocksdb:///home/jrad/oxide_db \
    --db-tuning '
        max_background_jobs=16,
        write_buffer_size=536870912,
        max_write_buffer_number=8,
        min_write_buffer_number_to_merge=2,
        block_cache_size=17179869184,
        delayed_write_rate=1073741824
    '
