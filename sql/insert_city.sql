INSERT INTO cities("department_code", "insee_code", "zip_code", "name", "lat", "lon")
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *;
