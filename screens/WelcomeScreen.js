import React from 'react';
import { View, Text, StyleSheet, ImageBackground, TouchableOpacity } from 'react-native';

export default function WelcomeScreen({ navigation }) {
  return (
    <ImageBackground source={require('../assets/pexels-fotios-photos-2304253.jpg')} style={styles.background}>
      <View style={styles.overlay}>
        <View style={styles.container}>
          <Text style={styles.title}>EcoLegacy</Text>
          <Text style={styles.description}>
            Welcome to EcoLegacy! Discover sustainability initiatives and make a positive impact on the environment.
          </Text>
          <Text style={styles.subDescription}>
            Explore projects, chat with EcoBot, translate content, and more...
          </Text>
          <TouchableOpacity style={styles.button} onPress={() => navigation.navigate('Main')}>
            <Text style={styles.buttonText}>Get Started ➔</Text>
          </TouchableOpacity>
        </View>
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
  overlay: {
    flex: 1,
    width: '100%',
    backgroundColor: 'rgba(0,0,0,0.5)',
  },
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: 20,
  },
  title: {
    fontSize: 36,
    color: 'white',
    fontWeight: 'bold',
    textAlign: 'center',
    marginBottom: 20,
  },
  description: {
    fontSize: 18,
    color: 'white',
    textAlign: 'center',
    marginVertical: 10,
  },
  subDescription: {
    fontSize: 16,
    color: 'white',
    textAlign: 'center',
    marginBottom: 40,
  },
  button: {
    backgroundColor: 'rgba(255, 255, 255, 0.8)',
    paddingVertical: 10,
    paddingHorizontal: 20,
    borderRadius: 25,
  },
  buttonText: {
    fontSize: 18,
    color: '#000',
  },
});
