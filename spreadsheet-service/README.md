# Spreadsheet Microservice

Rust-based microservice for spreadsheet functionality with SQLite database.

## Features

- RESTful API for spreadsheet operations
- SQLite database for persistence
- Formula calculation (placeholder - to be replaced with IronCalc)
- Excel import/export (to be implemented with calamine/umya-spreadsheet)

## Setup

1. Install Rust dependencies:
```bash
cd spreadsheet-service
cargo build
```

2. Configure environment:
```bash
cp .env.example .env
```

3. Run the service:
```bash
cargo run
```

The service will start on `http://127.0.0.1:8080`

## API Endpoints

### Health Check
- `GET /api/health` - Service health status

### Sheets
- `GET /api/sheets` - List all sheets
- `POST /api/sheets` - Create a new sheet
- `GET /api/sheets/:id` - Get a specific sheet
- `PUT /api/sheets/:id` - Update a sheet
- `DELETE /api/sheets/:id` - Delete a sheet

### Cells
- `GET /api/sheets/:id/cells` - List all cells in a sheet
- `POST /api/sheets/:id/cells` - Create a new cell
- `GET /api/sheets/:id/cells/:row/:col` - Get a specific cell
- `PUT /api/sheets/:id/cells/:row/:col` - Update a cell
- `DELETE /api/sheets/:id/cells/:row/:col` - Delete a cell

### Formula
- `POST /api/sheets/:id/formula` - Calculate a formula

### Files
- `POST /api/files/import` - Import Excel file (TODO)
- `GET /api/files/export/:id` - Export Excel file (TODO)

## Database Schema

### sheets
- id (TEXT, PRIMARY KEY)
- name (TEXT, NOT NULL)
- created_at (TEXT, NOT NULL)
- updated_at (TEXT, NOT NULL)

### cells
- id (TEXT, PRIMARY KEY)
- sheet_id (TEXT, NOT NULL, FOREIGN KEY)
- row (INTEGER, NOT NULL)
- col (INTEGER, NOT NULL)
- value (TEXT)
- formula (TEXT)
- style (TEXT)
- created_at (TEXT, NOT NULL)
- updated_at (TEXT, NOT NULL)
- UNIQUE(sheet_id, row, col)

## TODO

- [ ] Implement IronCalc for formula calculation
- [ ] Implement Excel import with calamine
- [ ] Implement Excel export with umya-spreadsheet
- [ ] Add authentication/authorization
- [ ] Add WebSocket support for real-time collaboration
- [ ] Add comprehensive tests
