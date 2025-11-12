# Frontend

Proof explorer dashboard built with React + Vite + TypeScript + Tailwind CSS.

## Features

- **Dashboard**: Overview of rollup state and statistics
- **Batches**: View transaction batches and their status
- **Proofs**: Explore STARK proofs and Bitcoin commitments
- **Challenges**: Monitor active challenges and dispute resolution

## Development

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

The frontend will be available at `http://localhost:5173`.

## API Integration

The frontend connects to the backend API at `http://localhost:3000` via proxy configuration in `vite.config.ts`.

## Tech Stack

- **React 18**: UI framework
- **TypeScript**: Type safety
- **Vite**: Build tool
- **Tailwind CSS**: Styling
- **Axios**: HTTP client
- **React Router**: Navigation
- **Lucide React**: Icons
