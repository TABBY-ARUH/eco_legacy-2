const express = require('express');
const swaggerUi = require('swagger-ui-express');
const swaggerJsdoc = require('swagger-jsdoc');
const routes = require('./routes'); // Path to your routes

const app = express();
const port = 3000;

// Swagger definition
const swaggerDefinition = {
  openapi: '3.0.0',
  info: {
    title: 'EcoLegacy API',
    version: '1.0.0',
    description: 'API documentation for EcoLegacy.',
  },
  servers: [
    {
      url: `http://localhost:${port}`,
    },
  ],
};

// Options for the swagger docs
const options = {
  swaggerDefinition,
  apis: ['./routes/index.js'], // Ensure this path matches your file structure
};

// Initialize swagger-jsdoc
const swaggerSpec = swaggerJsdoc(options);

app.use('/api-docs', swaggerUi.serve, swaggerUi.setup(swaggerSpec));
app.use('/', routes);

app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
});


