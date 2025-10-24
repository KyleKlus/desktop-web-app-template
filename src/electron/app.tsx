import React from 'react';
import { createRoot } from 'react-dom/client';
import Example from '../not-electron/components/Example';

const root = createRoot(document.body);
root.render(<Example />);