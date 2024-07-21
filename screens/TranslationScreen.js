import React from 'react';
import { View, Text, Button, StyleSheet, ImageBackground } from 'react-native';

export default function TranslationScreen() {
  return (
    <ImageBackground source={require('../assets/background.jpg')} style={styles.background}>
      <View style={styles.container}>
        <Text style={styles.title}>Language Translation</Text>
        <Text style={styles.description}>
          EcoLegacy's translation feature ensures language isn't a barrier to engaging with sustainability...
        </Text>
        <Button title="Confirm language" onPress={() => {}} />
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
