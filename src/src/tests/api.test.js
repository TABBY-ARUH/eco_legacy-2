// src/tests/api.test.js
const request = require('supertest');
const express = require('express');
const app = require('../server'); // Adjust the path as needed

describe('API Endpoints', () => {
  // Test GET /projects
  it('should retrieve a list of all sustainability projects', async () => {
    const response = await request(app).get('/projects');
    expect(response.statusCode).toBe(200);
    expect(response.body).toBeInstanceOf(Array);
  });

  // Test POST /projects
  it('should create a new project', async () => {
    const newProject = { name: 'Test Project', description: 'A test project' };
    const response = await request(app).post('/projects').send(newProject);
    expect(response.statusCode).toBe(201);
    expect(response.body.name).toBe(newProject.name);
  });

  // Test GET /projects/{id}
  it('should retrieve a specific project by ID', async () => {
    const newProject = { name: 'Test Project', description: 'A test project' };
    const createResponse = await request(app).post('/projects').send(newProject);
    const projectId = createResponse.body.id;

    const response = await request(app).get(`/projects/${projectId}`);
    expect(response.statusCode).toBe(200);
    expect(response.body.id).toBe(projectId);
  });

  // Test PUT /projects/{id}
  it('should update a project by ID', async () => {
    const newProject = { name: 'Test Project', description: 'A test project' };
    const createResponse = await request(app).post('/projects').send(newProject);
    const projectId = createResponse.body.id;

    const updatedProject = { name: 'Updated Project', description: 'An updated test project' };
    const response = await request(app).put(`/projects/${projectId}`).send(updatedProject);
    expect(response.statusCode).toBe(200);
    expect(response.body.name).toBe(updatedProject.name);
  });

  // Test DELETE /projects/{id}
  it('should delete a project by ID', async () => {
    const newProject = { name: 'Test Project', description: 'A test project' };
    const createResponse = await request(app).post('/projects').send(newProject);
    const projectId = createResponse.body.id;

    const deleteResponse = await request(app).delete(`/projects/${projectId}`);
    expect(deleteResponse.statusCode).toBe(204);

    const getResponse = await request(app).get(`/projects/${projectId}`);
    expect(getResponse.statusCode).toBe(404);
  });

  // Test GET /categories
  it('should retrieve all categories', async () => {
    const response = await request(app).get('/categories');
    expect(response.statusCode).toBe(200);
    expect(response.body).toBeInstanceOf(Array);
  });

  // Test GET /categories/{id}
  it('should retrieve a specific category by ID', async () => {
    const response = await request(app).get('/categories/1'); // Adjust ID as needed
    expect(response.statusCode).toBe(200);
  });

  // Test POST /ecobot
  it('should interact with the EcoBot chatbot', async () => {
    const response = await request(app).post('/ecobot').send({ message: 'Hello' });
    expect(response.statusCode).toBe(200);
    // Add additional checks based on the response structure
  });

  // Test POST /translate
  it('should translate text to a specified language', async () => {
    const response = await request(app).post('/translate').send({ text: 'Hello', language: 'es' });
    expect(response.statusCode).toBe(200);
    // Add additional checks based on the response structure
  });
});
