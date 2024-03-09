 INSERT INTO agents
            (account_id,
             symbol,
             headquarters,
             credits,
             starting_faction,
             ship_count,
             token,
             created_at,
             updated_at)
VALUES      (?1,
             ?2,
             ?3,
             ?4,
             ?5,
             ?6,
             ?7,
             CURRENT_TIMESTAMP,
             CURRENT_TIMESTAMP)
            
