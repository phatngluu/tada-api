select
   100 * (extract (epoch from (max (time) at time zone 'UTC')) - extract (epoch from (min (time) at time zone 'UTC')))
      / (extract (epoch from (now () at time zone 'UTC')) - extract (epoch from (min (time) at time zone 'UTC')))
  as sync_percent
  from block ;

-- 77.6

-- select * from pool_update
--     where registered_tx_id in (select max(registered_tx_id) from pool_update group by hash_id)
--     and not exists
--       ( select * from pool_retire where pool_retire.hash_id = pool_update.hash_id
--           and pool_retire.retiring_epoch <= (select max (epoch_no) from block)
--       ) ;