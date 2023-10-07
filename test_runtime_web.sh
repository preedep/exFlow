curl -v -X POST http://localhost:8082/api/v1/run_pipeline \
   -H 'Content-Type: application/json' \
   -d '{
      "subscription_id":"2ad6d4fd-dcef-4a30-86c7-becd50d38034",
       "resource_group_name":"NICK-RG-SEA-001",
       "factory_name":"DevFactory001",
       "pipeline_name":"pipeline_parallel_function"
    }'