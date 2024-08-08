import React from 'react';
import { View, Text, StyleSheet, ImageBackground, TouchableOpacity, Alert, Linking } from 'react-native';
import { FontAwesome5, MaterialIcons } from '@expo/vector-icons';

export default function MainScreen({ navigation }) {
  const handleExplorePress = () => {
    // navigate to an Explore screen or show an alert
    Alert.alert("Explore App", "This button will take you to the Explore App section.");
  };

  const handleDownloadPress = () => {
    //  open a download URL
    const url = "https://example.com/download"; // Replace with your actual download URL
    Linking.openURL(url).catch((err) => console.error("Couldn't load page", err));
  };

  return (
    <ImageBackground source={require('../assets/pexels-fotios-photos-2304253.jpg')} style={styles.background}>
      <View style={styles.overlay}>
        <View style={styles.container}>
          <TouchableOpacity style={styles.button} onPress={() => navigation.navigate('Project')}>
            <FontAwesome5 name="project-diagram" size={24} color="white" />
            <Text style={styles.buttonText}>Project</Text>
          </TouchableOpacity>
          <TouchableOpacity style={styles.button} onPress={() => navigation.navigate('Chat')}>
            <MaterialIcons name="chat" size={24} color="white" />
            <Text style={styles.buttonText}>Chatbot</Text>
          </TouchableOpacity>
          <TouchableOpacity style={styles.button} onPress={() => navigation.navigate('Translation')}>
            <MaterialIcons name="translate" size={24} color="white" />
            <Text style={styles.buttonText}>Translation</Text>
          </TouchableOpacity>
        </View>
        <View style={styles.footer}>
          <TouchableOpacity style={styles.footerButton} onPress={handleExplorePress}>
            <Text style={styles.footerButtonText}>Explore App</Text>
          </TouchableOpacity>
          <TouchableOpacity style={styles.footerButton} onPress={handleDownloadPress}>
            <Text style={styles.footerButtonText}>Download</Text>
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
  button: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: '#003f5c',
    paddingVertical: 15,
    paddingHorizontal: 25,
    borderRadius: 25,
    marginVertical: 10,
    width: '80%',
  },
  buttonText: {
    color: 'white',
    fontSize: 18,
    marginLeft: 10,
  },
  footer: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    marginBottom: 40,
  },
  footerButton: {
    backgroundColor: 'rgba(255, 255, 255, 0.8)',
    paddingVertical: 10,
    paddingHorizontal: 20,
    borderRadius: 25,
  },
  footerButtonText: {
    fontSize: 16,
    color: '#000',
  },
});
