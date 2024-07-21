import React from 'react';
import { View, Text, StyleSheet, ImageBackground } from 'react-native';

export default function ChatScreen() {
  return (
    <ImageBackground source={require('../assets/background.jpg')} style={styles.background}>
      <View style={styles.container}>
        <Text style={styles.title}>Chat with EcoBot</Text>
        <Text style={styles.description}>Hey there! Ready to explore the limitless possibilities...</Text>
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
  description: {
    fontSize: 18,
    color: 'white',
    textAlign: 'center',
    marginVertical: 20,
  },
});
