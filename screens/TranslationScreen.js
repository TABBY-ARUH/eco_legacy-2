import React from 'react';
import { View, Text, StyleSheet, ImageBackground, TextInput, TouchableOpacity } from 'react-native';
import { MaterialIcons } from '@expo/vector-icons';

export default function TranslationScreen() {
  return (
    <ImageBackground source={require('../assets/pexels-fotios-photos-2304253.jpg')} style={styles.background}>
      <View style={styles.overlay}>
        <View style={styles.header}>
          <TouchableOpacity onPress={() => navigation.goBack()}>
            <MaterialIcons name="arrow-back" size={24} color="white" />
          </TouchableOpacity>
        </View>
        <View style={styles.container}>
          <Text style={styles.title}>Language Translation</Text>
          <View style={styles.languageSelector}>
            <TextInput
              style={styles.languageInput}
              placeholder="Select Language"
              placeholderTextColor="#aaa"
            />
            <MaterialIcons name="arrow-drop-down" size={24} color="white" />
          </View>
          <Text style={styles.description}>
            "EcoLegacy's translation feature ensures language isn't a barrier to engaging with sustainability. By promoting global participation and understanding, it fosters collaboration for a greener future."
          </Text>
          <TouchableOpacity style={styles.button}>
            <Text style={styles.buttonText}>Confirm language</Text>
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
  header: {
    position: 'absolute',
    top: 40,
    left: 20,
  },
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: 20,
  },
  title: {
    fontSize: 24,
    color: 'white',
    fontWeight: 'bold',
    textAlign: 'center',
    marginBottom: 20,
  },
  languageSelector: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: 'rgba(255, 255, 255, 0.3)',
    paddingHorizontal: 10,
    borderRadius: 10,
    marginBottom: 20,
  },
  languageInput: {
    flex: 1,
    color: 'white',
    paddingVertical: 10,
  },
  description: {
    fontSize: 18,
    color: 'white',
    textAlign: 'center',
    marginVertical: 20,
  },
  button: {
    backgroundColor: 'rgba(255, 255, 255, 0.8)',
    paddingVertical: 10,
    paddingHorizontal: 20,
    borderRadius: 25,
    alignItems: 'center',
    justifyContent: 'center',
    marginVertical: 20,
  },
  buttonText: {
    fontSize: 18,
    color: '#000',
  },
});
