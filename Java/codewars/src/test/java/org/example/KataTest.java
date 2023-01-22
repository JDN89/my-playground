package org.example;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class KataTest {

      Kata kata;

    @BeforeEach
    void setUp() {
        kata = new Kata();
    }


    @Test
    public void exampleTests() {
        assertEquals(1, kata.quarterOf(3),"Should be in first quarter") ;
        assertEquals(3, kata.quarterOf(8),"Should be in first quarter") ;
        assertEquals(4, kata.quarterOf(11),"Should be in first quarter") ;
    }

}