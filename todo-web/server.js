
// expressJS
import express from "express";
const app = express();

// Static Serving
app.use(express.static("public"));

// Middleware
app.use(express.json()); // Body JSON Parsing
app.use(express.urlencoded({ extended: false })); // Query Parsing

// pugJS
app.set("views", "templates");
app.set("view engine", "pug");

// Request Logging
app.use("*", (req, res, next) => {
  res.on("finish", () => {
    console.log(`${req.method} ${req.originalUrl} - ${res.statusCode}`);
  });

  next();
});

// Server Constants
const PORT = 8081;


/*
 *  Routing
 */

// Main
app.get(["/", "/inbox"], async (req, res) => {
  res.render("layout.pug");
});

// 404 Not Found Catch-all
app.get("*", (_, res) => {
  res.status(404).render("404.pug");
});


/*
 *  Server Intitialization
 */

// Server Listener
app.listen(PORT, () => {
  console.log(`Todo List Web Server started on Port ${PORT}`);
});
