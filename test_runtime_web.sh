curl -v -X POST http://localhost:8082/api/v1/run_pipeline \
   -H 'Content-Type: application/json' \
   -d '{
      "subscriptionId":"2ad6d4fd-dcef-4a30-86c7-becd50d38034",
       "resourceGroupName":"NICK-RG-SEA-001",
       "factoryName":"DevFactory001",
       "pipelineName":"pipeline_parallel_function"
    }'