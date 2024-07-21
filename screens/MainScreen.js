import React from 'react';
import { View, Button, StyleSheet, ImageBackground } from 'react-native';

export default function MainScreen({ navigation }) {
  return (
    <ImageBackground source={require('')} style={styles.background}>
      <View style={styles.container}>
        <Button title="Project" onPress={() => navigation.navigate('Project')} />
        <Button title="Chatbot" onPress={() => navigation.navigate('Chat')} />
        <Button title="Translation" onPress={() => navigation.navigate('Translation')} />
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
});
