
REQUEST='{"input_folder":"/home/julianramirezj/input-backup-2","output_folder":"/home/julianramirezj/backup-system-api/backup","pass":"secret"}'
curl -v -i -X POST -H 'Content-Type: application/json' 'http://127.0.0.1:8000/backup/create' -d "${REQUEST}"
