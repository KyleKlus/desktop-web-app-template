import React from 'react';
import App from './App';
import ReactDOM from 'react-dom/client';

// Setup react
const rootElement = document.getElementById('root');
if (!rootElement) throw new Error('No root element found');

const root = ReactDOM.createRoot(rootElement);
root.render(
  <React.StrictMode>
    <script src="https://accounts.google.com/gsi/client" async></script>
    <App />
  </React.StrictMode>,
);
