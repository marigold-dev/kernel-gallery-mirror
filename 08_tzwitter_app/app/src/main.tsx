import ReactDOM from 'react-dom/client';
import './index.css';
import Index from './pages';
import React from 'react';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <Index />
  </React.StrictMode>,
);
