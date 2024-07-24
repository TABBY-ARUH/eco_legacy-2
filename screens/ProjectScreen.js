import React, { useEffect, useState } from 'react';
import { View, Text, StyleSheet, ImageBackground, FlatList, TextInput, Button, Alert } from 'react-native';

export default function ProjectScreen() {
  const [projects, setProjects] = useState([]);
  const [newProject, setNewProject] = useState({ name: '', description: '', year: '' });

  useEffect(() => {
    fetch('http://localhost:8081/projects')
      .then(response => response.json())
      .then(data => setProjects(data))
      .catch(error => console.error('Error fetching projects:', error));
  }, []);

  const handleCreateProject = () => {
    fetch('http://localhost:8081/projects', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(newProject),
    })
      .then(response => response.json())
      .then(data => {
        setProjects([...projects, data]);
        setNewProject({ name: '', description: '', year: '' });
        Alert.alert('Project created successfully!');
      })
      .catch(error => {
        console.error('Error creating project:', error);
        Alert.alert('Failed to create project.');
      });
  };

  return (
    <ImageBackground source={require('../assets/pexels-fotios-photos-2304253.jpg')} style={styles.background}>
      <View style={styles.container}>
        <Text style={styles.title}>Explore Sustainability Projects</Text>
        <FlatList
          data={projects}
          keyExtractor={item => item.id.toString()}
          renderItem={({ item }) => (
            <View style={styles.projectItem}>
              <Text style={styles.projectName}>{item.name}</Text>
              <Text style={styles.projectDescription}>{item.description}</Text>
              <Text style={styles.projectYear}>{item.year}</Text>
            </View>
          )}
        />
        <TextInput
          style={styles.input}
          placeholder="Project Name"
          value={newProject.name}
          onChangeText={text => setNewProject({ ...newProject, name: text })}
        />
        <TextInput
          style={styles.input}
          placeholder="Project Description"
          value={newProject.description}
          onChangeText={text => setNewProject({ ...newProject, description: text })}
        />
        <TextInput
          style={styles.input}
          placeholder="Project Year"
          value={newProject.year}
          onChangeText={text => setNewProject({ ...newProject, year: text })}
        />
        <Button title="Create Project" onPress={handleCreateProject} />
      </View>
    </ImageBackground>
  );
}

const styles = StyleSheet.create({
  background: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  container: {
    padding: 20,
    backgroundColor: 'rgba(0,0,0,0.5)',
    borderRadius: 10,
  },
  title: {
    fontSize: 24,
    color: 'white',
    fontWeight: 'bold',
    textAlign: 'center',
  },
  projectItem: {
    marginVertical: 10,
  },
  projectName: {
    fontSize: 20,
    color: 'white',
  },
  projectDescription: {
    fontSize: 16,
    color: 'white',
  },
  projectYear: {
    fontSize: 14,
    color: 'white',
  },
  input: {
    backgroundColor: 'white',
    padding: 10,
    marginVertical: 5,
    borderRadius: 5,
  },
});
