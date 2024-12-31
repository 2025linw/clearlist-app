
// expressJS
const express = require('express');
const app = express();

// Request Logging
app.use('*', (req, _, next) => {
  console.log(`${req.method} ${req.originalUrl} HTTP/${req.httpVersion}`);

  next();
});

// Static Serving
app.use(express.static('public'));

// Middleware
app.use(express.json()); // Body JSON Parsing
app.use(express.urlencoded({ extended: false })); // Query Parsing

// pugJS
app.set('views', 'templates');
app.set('view engine', 'pug');

// Server Constants
const PORT = 8081;


/*
 *  Routing
 */

// Main
app.get(['/', '/inbox'], async (req, res) => {
  res.render('layout.pug');
});

// 404 Not Found Catch-all
app.get('*', (_, res) => {
  res.status(404).render('404.pug');
});

app.listen(PORT, () => {
  console.log(`Todo List Web Server started on Port ${PORT}`);
});
