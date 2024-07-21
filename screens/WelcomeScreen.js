import React from 'react';
import { View,Text, Button, StyleSheet, ImageBackground } from 'react-native';


export default function WelcomeScreen({navigation}) {
    return (
        <ImageBackground source={require('')} style={styles.background}>
            <View style={styles.background}>
                <Text style={styles.title}>EcoLegacy</Text>
                <Text style={styles.description}>
                    Welcome to EcoLegacy! Discover sustainability initiatives and make a positive impact on the environment.
                </Text>
                <Button title='Get Started' onPress={() => navigation.navigate('Main')} />
            </View>
        </ImageBackground>
    );
};

const styles = StyleSheet.create({
    background: {
        flex: 1,
        justifyContent: 'center',
        alignItems: 'center',
    },
    container: {
        padding: '20',
        backgroundColor: 'rgba(0,0,0.5)',
        borderRadius: 10,
    },
    title: {
        fontSize: 30,
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